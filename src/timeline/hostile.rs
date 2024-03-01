use std::cell::RefCell;
use std::rc::Rc;
// use std::collections::HashMap;
use serde_json::Value;
// use serde_json::Value;
use super::Event;
use crate::calculator::Calculator;
use crate::frame::Frame;
use crate::utils::error::ConfigParseError;
type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;
#[derive(Debug)]
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
        e.next_point = c.route[self.enemy_route][1];
        e.identifier=f.next_id;
        f.next_id+=1;
        f.enemy_set.push(Rc::new(RefCell::new(e)));
    }
}

impl EnemyPlaceEvent{
    pub fn new(v:&Value)->Result<EnemyPlaceEvent>{
        let enemy_key=String::from(v[2].as_str().ok_or(ConfigParseError("Enemy key can't translate to str in timeline".into()))?);
        let enemy_route=v[3].as_u64().ok_or(ConfigParseError("Enemy route can't translate to u64 in timeline".into()))? as usize;
        Ok(EnemyPlaceEvent{
            enemy_key,
            enemy_route
        })
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
