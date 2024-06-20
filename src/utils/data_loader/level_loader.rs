use std::path::Path;
use std::path::PathBuf;

use egui::Layout;
use serde_json::Value;
use serde::{ Deserialize,Serialize };

use crate::map::tile::DEPLOY_HIGH;
use crate::map::tile::DEPLOY_LOW;
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
struct OfficalLevelData{
    pub(super) options:Value,
    pub(super) mapData:OfficalMapData,
    pub(super) routes:Vec<OfficalRoute>,
    pub(super) enemyDbRefs:Vec<OfficalEnemyDbRef>,
    pub(super) waves:Vec<OfficalWave>,
    pub(super) randomSeed:u32,
}

#[derive(Deserialize,Default,Debug,Clone)]
struct OfficalMapData{
    pub(super) map:Vec<Vec<u64>>,
    pub(super) tiles:Vec<OfficalTile>,
}

#[derive(Deserialize,Default,Debug,Clone,Copy)]
struct OfficalTile{
    pub(super) tileKey:TileKey,
    pub(super) heightType:TileHeight,
    pub(super) buildableType:TileBuildable,
    pub(super) passableMask:TilePassable,
    // pub(super) blackboard:String,
    // pub(super) effects:Vec<OfficalEffect>,
    // it also has a palyerSideMask,but all is value is "ALL",so i ignore it
}

#[derive(Deserialize,Default,Debug)]
struct OfficalRoute{
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
struct OfficalEnemyDbRef{
    pub(super) useDb:bool,
    pub(super) id:String,
    pub(super) level:i32,
    pub(super) overwrittenData:Option<Value>,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalWave{
    pub(super) preDelay:f32,
    pub(super) postDelay:f32,
    pub(super) fragments:Vec<OfficalWaveFragment>,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalWaveFragment{
    pub(super) preDelay:f32,
    pub(super) actions:Vec<OfficalWaveAction>,

}

#[derive(Deserialize,Default,Debug)]
struct OfficalWaveAction{
    // story,diaplay enemy info,and so on,it seems not related to the enmey behavior
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
impl Into<LayoutCode> for OfficalTile{
    fn into(self)->LayoutCode{
        let mut c=0;
        c |= self.tileKey as u64;
        // l don't know why HG set both of heightype and buildabletype
        // c |= match self.heightType {
        //     TileHeight::Lowland => DEPLOY_LOW,
        //     TileHeight::Highland => DEPLOY_HIGH,
        //     _ => 0,
        // };
        c |= match self.buildableType {
            TileBuildable::Melee => DEPLOY_LOW,
            TileBuildable::Ranged => DEPLOY_HIGH,
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
impl Into<Map> for OfficalMapData{
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
    fn load_level(&self, level_name: String)->Result<OfficalLevelData>{
        let path = self.path.join("levels");
        let level_file = level_name + ".json";
        let file_path = find_file_in_dir(&path, &level_file)?;
        let level_json = load_json_file(file_path)?;
        let level = serde_json::from_value::<OfficalLevelData>(level_json)?; 
        Ok(level)
    }
    fn load_map(&self,level:&OfficalLevelData)->Result<Map>{
        let map:Map= level.mapData.clone().into();
        Ok(map)
    }
}

#[cfg(test)]
mod test{
    use serde_json::from_value;

    use super::*;
    #[test]
    fn test_level_loader(){
        let path = Path::new("data/levels/obt");
        let file_name = "level_main_01-07.json";
        let result = find_file_in_dir(path, file_name).unwrap();
        let level = load_json_file(result).unwrap();
        let data = from_value::<OfficalLevelData>(level).unwrap();
        println!("{:?}",data);
    }

    fn find_all_file_in_dir(dir:&Path,list:&mut Vec<String>){
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

    fn get_value_by_key(path:&Path,list:&mut Vec<String>){
        let json=load_json_file(path).unwrap();
        if let Ok(data) = from_value::<OfficalLevelData>(json)
        {
            for i in data.waves.into_iter(){
                for j in i.fragments.into_iter(){
                    for k in j.actions.into_iter(){
                        if !list.contains(&k.actionType){
                            list.push(k.actionType);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn find_all_value(){
        let mut value_list=Vec::<String>::new();
        let path = Path::new("data/levels/obt/main");
        find_all_file_in_dir(path,&mut value_list);
        println!("{:?}",value_list); 
    }

}