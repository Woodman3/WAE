use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::i64;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use egui::Layout;
use serde::de;
use serde_json::Value;
use serde::{ Deserialize,Serialize };

use crate::calculator::Calculator;
use crate::frame::EnemyRef;
use crate::frame::Frame;
use crate::map::tile::DEPLOY_HIGH;
use crate::map::tile::DEPLOY_LOW;
use crate::map::tile::DEPLOY_NONE;
use crate::map::tile::PASS_ALL;
use crate::map::tile::PASS_FLY;
use crate::map::Map;
use crate::utils::load_json_file;
use crate::utils::math::Grid;
use crate::utils::math::Point;
use super::Result;
use super::Loader;
use crate::map::tile::{LayoutCode,TileHeight,TileBuildable,TilePassable,TileKey};

#[derive(Deserialize,Default,Debug)]
struct OfficialLevelData{
    pub(super) options:Value,
    pub(super) mapData: OfficialMapData,
    pub(super) routes:Vec<OfficialRoute>,
    pub(super) enemyDbRefs:Vec<OfficialEnemyDbRef>,
    pub(super) waves:Vec<OfficialWave>,
    pub(super) randomSeed:u32,
}

#[derive(Deserialize,Default,Debug,Clone)]
struct OfficialMapData {
    pub(super) map:Vec<Vec<u64>>,
    pub(super) tiles:Vec<OfficialTile>,
}

#[derive(Deserialize,Default,Debug,Clone,Copy)]
struct OfficialTile {
    pub(super) tileKey:TileKey,
    pub(super) heightType:TileHeight,
    pub(super) buildableType:TileBuildable,
    pub(super) passableMask:TilePassable,
    // pub(super) blackboard:String,
    // pub(super) effects:Vec<OfficalEffect>,
    // it also has a palyerSideMask,but up to 2024/7/29, all its value is "ALL",so i ignore it
}

#[derive(Deserialize,Default,Debug)]
struct OfficialRoute {
    pub(super) motionMode:Route,
    pub(super) startPosition:Grid,
    pub(super) endPosition:Grid,
    pub(super) spawnRandomRange:Point,
    pub(super) spawnOffset:Point,
    pub(super) checkpoints:Value,
    pub(super) allowDiagonalMove: bool,
    pub(super) visitEveryTileCenter: bool,
    pub(super) visitEveryNodeCenter: bool,
    pub(super) visitEveryCheckPoint: bool,
}

#[derive(Serialize,Deserialize,Debug,Default,Clone,Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Route {
    #[default]
    ENum,
    Walk,
    Fly 
}

#[derive(Deserialize,Default,Debug)]
struct OfficialEnemyDbRef {
    pub(super) useDb:bool,
    pub(super) id:String,
    pub(super) level:i32,
    pub(super) overwrittenData:Option<Value>,
}

#[derive(Deserialize,Default,Debug)]
struct OfficialWave {
    pub(super) preDelay:f32,
    pub(super) postDelay:f32,
    pub(super) fragments:Vec<OfficialWaveFragment>,
}

#[derive(Deserialize,Default,Debug)]
struct OfficialWaveFragment {
    pub(super) preDelay:f32,
    pub(super) actions:Vec<OfficialWaveAction>,

}

#[derive(Deserialize,Default,Debug)]
struct OfficialWaveAction {
    // story,display enemy info,and so on,it seems not related to the enemy behavior
    pub(super) actionType:String,
    pub(super) preDelay:f32,
    pub(super) routeIndex:u32,

}

// enun OfficalP

fn find_file_in_dir(dir: &Path, file_name: &str) -> Result<String> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.file_name().map_or(false, |f| f == file_name) {
                return Ok(path.to_string_lossy().into_owned());
            } else if path.is_dir() {
                if let Ok(found) = find_file_in_dir(&path, file_name){
                    return Ok(found)
                }
            }
        }
    }
    Err("File not found".into())
}
impl Into<LayoutCode> for OfficialTile {
    fn into(self)->LayoutCode{
        let mut c=0;
        c |= self.tileKey as u64;
        // up to 2024/7/29,all of (TileBuildable,TilePassable)  have only 4 value :
        // [(None, Highland), (None, Lowland), (Melee, Lowland), (Ranged, Highland)]
        c |= match self.heightType {
            TileHeight::Lowland => DEPLOY_LOW,
            TileHeight::Highland => DEPLOY_HIGH,
        };
        c |= match self.buildableType {
            TileBuildable::None => DEPLOY_NONE,
            _ => 0,
        };
        c |= match self.passableMask {
            TilePassable::FlyOnly => PASS_FLY,
            TilePassable::All => PASS_ALL,
            _ => 0,
        }; 
        c
    }
}

//todo:there stil have some problem 
impl Into<Map> for OfficialMapData {
    fn into(self)->Map{
        let width = self.map[0].len() ;
        let height = self.map.len();
        let mut layout=vec![vec![0; width]; height]; ;
        for i in 0..height{
            for j in 0..width{
                layout[i][j]=self.tiles[self.map[i][j] as usize].into();
            }
        }
        Map{
            width:width as u32,
            height:height as u32,
            layout,
            enemy:Vec::new(),
            operator:Vec::new(),
        }
    }
}
impl Loader{
    fn find_level(&self, level_name: String)->Result<OfficialLevelData>{
        let path = self.path.join("levels");
        let level_file = level_name + ".json";
        let file_path = find_file_in_dir(&path, &level_file)?;
        let level_json = load_json_file(file_path)?;
        let level = serde_json::from_value::<OfficialLevelData>(level_json)?;
        Ok(level)
    }
    fn load_map(&self,level:&OfficialLevelData)->Result<Map>{
        let map:Map= level.mapData.clone().into();
        Ok(map)
    }

    pub(crate) fn load_level(&self,level_name:String)->Result<Calculator>{
        let level = self.find_level(level_name)?;
        let map = self.load_map(&level)?;
        let mut enemy_initial=HashMap::new();
        for e in level.enemyDbRefs.iter(){
            let enemy = self.load_official_enemy(&e.id,e.level as usize)?;
            enemy_initial.insert(e.id.clone(),enemy );
        }
        
         
        let f = Frame{
            map,
            ..Default::default()
        };
        let c= Calculator{
            frame_vec:vec![f],
            time_remain:i64::MAX,
            last_enemy_time:0,
            star:-1,
            time_line:VecDeque::new(),
            route:Vec::new(),
            enemy_initial,
        };
        return Ok(c);
        todo!("load level")
    }
}

#[cfg(test)]
mod test{
    use serde_json::from_value;

    use super::*;

    fn find_all_file_in_dir(dir:&Path,list:&mut Vec<(TileBuildable,TileHeight)>){
        if dir.is_dir(){
            for entry in std::fs::read_dir(dir).unwrap(){
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file(){
                    get_value_by_key(&path, list);
                }else if path.is_dir(){
                    find_all_file_in_dir(&path,list);
                }
            }
        }
    }

    fn get_value_by_key(path:&Path,list:&mut Vec<(TileBuildable,TileHeight)>){
        let json=load_json_file(path).unwrap();
        if let Ok(data) = from_value::<OfficialLevelData>(json)
        {
            for t in data.mapData.tiles.into_iter(){
                if !list.contains(&(t.buildableType,t.heightType)){
                    list.push((t.buildableType,t.heightType));
                }
            }
        }
    }

    #[test]
    fn find_all_value(){
        // let mut value_list=Vec::<(TileBuildable,TileHeight)>::new();
        let mut value_list = Vec::new();
        let path = Path::new("ArknightsGameData/zh_CN/gamedata/levels/obt");
        find_all_file_in_dir(path,&mut value_list);
        println!("{:?}",value_list); 
    }

}