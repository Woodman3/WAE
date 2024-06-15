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

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Deserialize,Default,Debug)]
struct LevelLoader{
    path:PathBuf,
    level:String,
    data:Value
}

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
    pub(super) map:Vec<Vec<i32>>,
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

impl LevelLoader{
    fn new<P: AsRef<Path>>(path: P, level:String)->LevelLoader{
        LevelLoader{
            path:path.as_ref().to_path_buf(),
            level:level,
            data:Value::Null,
        }
    }
    fn load(&mut self)->Result<()>{
        self.data = load_json_file(self.path.join(self.level.clone()+".json"))?;
        Ok(())
    }
    fn find_level(&self, level_name: String) -> Result<&Value> {
        let level_path = Self::find_file_in_dir(&self.path, &format!("{}.json", level_name))?;

        Err("Level not found".into())
        
    }
    fn find_file_in_dir(dir: &Path, file_name: &str) -> Result<String> {
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.file_name().map_or(false, |f| f == file_name) {
                    return Ok(path.to_string_lossy().into_owned());
                } else if path.is_dir() {
                    if let Ok(found) = Self::find_file_in_dir(&path, file_name){
                        return Ok(found)
                    }
                }
            }
        }
        Err("File not found".into())
    }

}
// impl Loader{
//     fn map_generate(&self)->Result<Map>{
        
//     }
// }

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
        let result = LevelLoader::find_file_in_dir(path, file_name).unwrap();
        println!("{:?}", result);
    }
}