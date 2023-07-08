use std::fs;
use std::error;
// use serde_json::{Value};
pub mod map;
pub mod blockinfo;
use map::Map;
type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;
// fn read_for_json(json_path:&str)->Result<Map>{
//     let map_path=json_path.to_owned()+"map.json";
//     let content = fs::read_to_string(map_path)?;
//         // .expect(&format!("can't get the path {file_path}"));
//     // let data:Value=serde_json::from_str(&content)?;
//     // .expect(&format!("can't read json,file content :\n {content} \n"));
//     let m:Map = serde_json::from_str(&content)?;
//         // .unwrap();
//     Ok(m)
//     // println!("{}");
//
// }
fn main() {
    // let m = crate::read_for_json("/home/archer/workspace/BEC/config/").unwrap();
    // println!("{:?}",m);
    let v=blockinfo::read_from_json("/home/archer/workspace/BEC/config/").unwrap();     
    println!("{:?}",v);
}
