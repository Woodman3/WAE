use serde::{Deserialize,Serialize};
type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;
#[derive(Serialize,Deserialize,Debug)]
pub struct Map{
    pub width:u64,
    pub height:u64,
    pub layout:Vec<Vec<u64>>
}

pub fn read_for_json(json_path:&str)->Result<Map>{
    let map_path=json_path.to_owned()+"map.json";
    let content = std::fs::read_to_string(map_path)?;
        // .expect(&format!("can't get the path {file_path}"));
    // let data:Value=serde_json::from_str(&content)?;
    // .expect(&format!("can't read json,file content :\n {content} \n"));
    let m:Map = serde_json::from_str(&content)?;
        // .unwrap();
    Ok(m)
    // println!("{}");
}
