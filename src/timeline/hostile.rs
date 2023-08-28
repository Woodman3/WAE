use std::rc::Rc;
// use std::collections::HashMap;
use serde::Deserialize;
// use serde_json::Value;
use super::Event;
use crate::calculator::Calculator;
use crate::frame::Frame;
// type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;
#[derive(Deserialize, Debug)]
pub struct EnemyPlaceEvent {
    enemy_key: String,
    enemy_route: usize,
}
impl Event for EnemyPlaceEvent {
    fn happen(&self, f: &mut Frame, c: &Calculator) {
        let mut e = c
            .enemy_initial
            .get(self.enemy_key.as_str())
            .cloned()
            .unwrap();
        e.route = Some(Rc::clone(&c.route[self.enemy_route]));
        e.location = c.route[self.enemy_route][0];
        e.target = c.route[self.enemy_route][1];
        f.enemy_set.push(e);
    }
}

// pub fn construct_from_json(json_path:&str)->Result<HashMap<String,Vec<Vec<u64>>>>{
//     let path=json_path.to_owned()+"hostile.json";
//     let content = std::fs::read_to_string(path)?;
//     let j:Value = serde_json::from_str(&content)?;
//
//     let route:HashMap<String,Vec<Vec<u64>>> = serde_json::from_value(j["route"])?.unwrap();
//     // println!("{:?}\n",j);
//     // for (_key,value) in j["Block"].as_object().unwrap() {
//     //     // println!("key : {:?}, value :{:?}",key,value);
//     //     let info = value["PlaceInfo"].as_str().unwrap();
//     //     let t=construct_place_info(&info);
//     //     let index=value["id"].as_u64().unwrap() as usize;
//     //     // println!("block is {t},index is {index}");
//     //     r[index]=t;
//     // }
//     Ok(r)
// }
