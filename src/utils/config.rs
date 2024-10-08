use super::load_json_file;
use serde::Deserialize;
use serde_json::Value;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn construct_info_from_json<T: for<'a> Deserialize<'a>>(
    json_path: &str,
    point: &str,
) -> Result<Vec<T>> {
    // let mut r =vec![T,1;len];
    let mut r = Vec::<T>::new();
    let content: String = std::fs::read_to_string(&json_path)?;
    let binding = serde_json::from_str::<Value>(&content)?;
    let j: &Value = binding.pointer(point).unwrap();
    // println!("{:?}\n",j);
    for (_key, value) in j.as_object().unwrap() {
        // println!("key : {:?}, value :{:?}",key,value);
        let info: T = serde_json::from_value(value[super::get_short_type_name::<T>()].clone())?;
        // let t=construct_block_info(&info);
        // let index=value["id"].as_u64().unwrap() as usize;
        // println!("block is {t},index is {index}");
        r.push(info);
    }
    Ok(r)
}
#[derive(Debug)]
pub struct Config {
    pub hostile: Value,
    pub enemy: Value,
    pub operator: Value,
    pub doctor: Value,
    pub map: Value,
    pub skill: Value,
    pub demo: Value,
}
impl Config {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Config> {
        let hostile = load_json_file(path.as_ref().join("hostile.json"))?;
        let enemy = load_json_file(path.as_ref().join("enemy.json"))?;
        let operator = load_json_file(path.as_ref().join("operator.json"))?;
        let doctor = load_json_file(path.as_ref().join("doctor.json"))?;
        let map = load_json_file(path.as_ref().join("map.json"))?;
        let skill = load_json_file(path.as_ref().join("skill_test.json"))?;
        let demo = load_json_file(path.as_ref().join("demo.json"))?;

        Ok(Config {
            hostile,
            enemy,
            operator,
            doctor,
            map,
            skill,
            demo,
        })
    }
}
