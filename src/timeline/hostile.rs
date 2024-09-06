use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
// use std::collections::HashMap;
use serde_json::Value;
// use serde_json::Value;
use crate::calculator::Calculator;
use crate::frame::Frame;
use crate::route::CheckPoint;
use crate::unit::skill::skill_schedule::SkillSchedule;
use crate::unit::Unit;
use crate::utils::error::ConfigParseError;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub(crate) struct EnemyPlaceEvent {
    pub(crate) enemy_key: String,
    pub(crate) enemy_route: usize,
}
impl EnemyPlaceEvent {
    pub(super) fn happen(&self, f: &mut Frame, c: &Calculator) {
        let mut e = c
            .enemy_initial
            .get(self.enemy_key.as_str())
            .cloned()
            .unwrap();
        e.route = Rc::clone(&c.route[self.enemy_route]);
        e.location = e.route.start;
        e.next_point = match e
            .route
            .checkpoints
            .iter()
            .position(|c| matches!(c, CheckPoint::Move(_)))
        {
            Some(p) => match e.route.checkpoints[p] {
                CheckPoint::Move(p) => p,
                _ => e.route.end,
            },
            None => e.route.end,
        };
        e.id = f.next_id;
        e.init();
        let default_skill = e.generate_default_attack_skill();
        let e = Rc::new(RefCell::new(e));
        let s = SkillSchedule{
            skill_block: vec![default_skill],
            skill_ready: vec![],
            skill_running: vec![],
        };
        f.next_id += 1;
        f.enemy_set.push(e);
    }
    pub(super) fn new(v: &Value) -> Result<EnemyPlaceEvent> {
        let enemy_key = String::from(v[2].as_str().ok_or(ConfigParseError(
            "Enemy key can't translate to str in timeline".into(),
        ))?);
        let enemy_route = v[3].as_u64().ok_or(ConfigParseError(
            "Enemy route can't translate to u64 in timeline".into(),
        ))? as usize;
        Ok(EnemyPlaceEvent {
            enemy_key,
            enemy_route,
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
