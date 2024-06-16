use std::path::Path;
use std::path::PathBuf;

use serde_json::Value;
use serde::Deserialize;

use crate::map::Map;
use crate::utils::load_json_file;
use crate::utils::math::Grid;
use crate::utils::math::Point;
use super::Result;
use super::Loader;
use crate::map::block::LayoutCode;

#[derive(Deserialize,Default,Debug)]
struct OfficalLevelData{
    pub(super) options:Value,
    pub(super) mapData:OfficalMapData,
    pub(super) routes:Vec<OfficalRoute>,
    pub(super) enemyDbRefs:Vec<OfficalEnemyDbRef>,
    pub(super) waves:Vec<OfficalWave>,
    pub(super) ranmdomSeed:u32,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalMapData{
    pub(super) map:Vec<Vec<u32>>,
    pub(super) tiles:Vec<OfficalTile>,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalTile{
    pub(super) tileKey:String,
    pub(super) heightType:String,
    pub(super) buildable:String,
    pub(super) passable:String,
    pub(super) playerSideMask:String,
    // pub(super) blackboard:String,
    // pub(super) effects:Vec<OfficalEffect>,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalRoute{
    pub(super) motionMode:String,
    pub(super) starPosition:Grid,
    pub(super) endPosition:Grid,
    pub(super) spawnRandomRange:Point,
    pub(super) spawnOffset:Point,
    pub(super) checkpoint:Value,
    pub(super) allowDiagonalMove: bool,
    pub(super) visitEveryTileCenter: bool,
    pub(super) visitEveryNodeCenter: bool,
    pub(super) visitEveryCheckPoint: bool,
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
    pub(super) actionType:String,
    pub(super) preDelay:f32,
    pub(super) postDelay:f32,
    pub(super) routeIndex:u32,

}

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
    fn into(self)->u8{
        let c:u8=0;
        todo!()
    }
}

impl Into<Map> for OfficalMapData{
    fn into(self)->Map{
        let width = self.map[0].len() as u32;
        let height = self.map.len() as u32;
        todo!("imple layout")
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
    fn load_map(&self,level:&Value)->Result<Map>{
        todo!()
    }
}

#[cfg(test)]
mod test{
    use super::*;
    // #[test]
    // fn test_level_loader(){
    //     let mut loader = LevelLoader::new("src/data/levels", "level1");
    //     loader.load().unwrap();
    //     println!("{:?}", loader.data);
    // }
    #[test]
    fn test_find_file_in_dir(){
        // let path = Path::new("data/levels");
        let path = Path::new("data/levels/obt");
        let file_name = "level_main_01-07.json";
        let result = find_file_in_dir(path, file_name).unwrap();
        let level = load_json_file(result).unwrap();
        println!("{:?}",level);
    }
}