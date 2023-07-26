use serde_json::Value;
use serde::Deserialize;

type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;
pub fn construct_info_from_json<T:for<'a> Deserialize<'a>>(json_path:&str,point:&str)->Result<Vec<T>>{
    // let mut r =vec![T,1;len];
    let mut r = Vec::<T>::new();
    let content:String = std::fs::read_to_string(&json_path)?;
    let binding = serde_json::from_str::<Value>(&content)?;
    let j:&Value = binding.pointer(point).unwrap();
    // println!("{:?}\n",j);
    for (_key,value) in j.as_object().unwrap() {
        // println!("key : {:?}, value :{:?}",key,value);
        let info:T = serde_json::from_value(value[super::get_short_type_name::<T>()].clone())?;
        // let t=construct_block_info(&info);
        // let index=value["id"].as_u64().unwrap() as usize;
        // println!("block is {t},index is {index}");
        r.push(info);
    }
    Ok(r)
}
#[derive(Debug)]
pub struct Config{
    pub hostile:Value
}
impl Config{
    pub fn new(json_path:&str)->Result<Config>{
        let content:String = std::fs::read_to_string(json_path.to_owned()+"hostile.json")?;
        let binding = serde_json::from_str::<Value>(&content)?;
        Ok(Config{hostile:binding})
    }
}
