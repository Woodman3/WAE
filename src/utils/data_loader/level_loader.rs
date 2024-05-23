use std::path::Path;
use std::path::PathBuf;

use serde_json::Value;

use crate::map::Map;
use crate::utils::load_json_file;
use crate::utils::math::Grid;
use crate::utils::math::Point;
use super::Result;
use super::Loader;

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
struct LevelLoader{
    path:PathBuf,
    level:String,
    data:Value
}

struct OfficalLevelData{
    pub(super) options:Value,
    pub(super) tiles:Vec<Vec<i32>>,
}

struct OfficalTile{
    pub(super) tileKey:String,
    pub(super) heightType:String,
    pub(super) buildable:String,
    pub(super) passable:String,
    pub(super) playerSideMask:String,
    // pub(super) blackboard:String,
    // pub(super) effects:Vec<OfficalEffect>,
}

struct OfficalRoute{
    pub(super) motionMode:String,
    pub(super) starPosition:Grid,
    pub(super) endPosition:Grid,
    pub(super) spawnRandomRange:Point,
    pub(super) spawnOffset:Point,
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
                    return Self::find_file_in_dir(&path, file_name)
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
        let path = Path::new("data/levels");
        let file_name = "level_main_01-07.json";
        let result = LevelLoader::find_file_in_dir(path, file_name);
        println!("{:?}", result);
    }
}