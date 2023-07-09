const VEC_LEN:usize=1024;
pub mod blockinfo;
pub mod placeinfo;
// use serde::{Serialize,DeSerialize}
// use serde_json;
// pub mod blockinfo;
// type Result<T> = std::result::Result<T,Box<dyn error::Error>>;
// pub trait Block {}
// // pub trait BlockInfo {}
// #[derive(Serialize,DeSerialize,Debug)]
//
// pub struct Ground {
//     info:BlockInfo,
//     id:i32
// }
// pub struct Start {
//     info:BlockInfo,
//     id:i32
// }
// impl Block for Ground {}
//
// pub fn Blocks(json_path:&str)->Vec<>{
//     let map_path=json_path.to_owned()+"blcok.json";
//     let content = fs::read_to_string(map_path)?;
//      
// }
