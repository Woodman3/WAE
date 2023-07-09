use serde_json::Value;
use serde::{Serialize,Deserialize};

type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;
#[derive(Serialize,Deserialize,Debug)]
struct BlockInfo{
    can_place:bool,
    can_pass:bool,
    can_drop:bool,
    has_effect:bool,
    can_turn:bool
}
pub const HAS_EFFECT:u64 = 1;
pub const CAN_PLACE:u64= 1<<1; 
pub const CAN_DROP:u64 =1<<2;
pub const CAN_TURN:u64 = 1<<3;
pub const CAN_PASS:u64 =1<<4;

fn construct_block_info(info:&BlockInfo)->u64{
    let mut r:u64=0;
    if info.has_effect{
        r|=HAS_EFFECT;
    }
    if info.can_place{
        r|=CAN_PLACE;
    }
    if info.can_drop{
        r|=CAN_DROP;
    }
    if info.can_turn{
        r|=CAN_TURN;
    }
    if info.can_pass{
        r|=CAN_PASS;
    }
    r
}
pub fn construct_block_info_from_json(json_path:&str)->Result<Vec<u64>>{
    let mut r =vec![0;super::VEC_LEN];
    let map_path=json_path.to_owned()+"block.json";
    let content = std::fs::read_to_string(map_path)?;
    let j:Value = serde_json::from_str(&content)?;
    // println!("{:?}\n",j);
    for (_key,value) in j["Block"].as_object().unwrap() {
        // println!("key : {:?}, value :{:?}",key,value);
        let info:BlockInfo = serde_json::from_value(value["BlockInfo"].clone())?;
        let t=construct_block_info(&info);
        let index=value["id"].as_u64().unwrap() as usize;
        // println!("block is {t},index is {index}");
        r[index]=t;
    }
    Ok(r)
}
