use crate::map::Map;
use super::Result;
use super::Loader;

struct LevelLoader{
    path:PathBuf,
    level:String,
    data:Value
}

impl LevelLoader{
    fn new<P: AsRef<Path>>(path: P, level:String)->LevelLoader{
        LevelLoader{
            path:path,
            level:level,
            data:Value::Null,
        }
    }
    fn load(&mut self)->Result<()>{
        self.data = load_json_file(self.path.as_ref().join(self.level+".json"))?;
        Ok(())
    }
}
// impl Loader{
//     fn map_generate(&self)->Result<Map>{
        
//     }
// }