use serde_json::Value;
pub const HIGH:u64=1;
pub const LOW:u64=1<<1;
type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;
fn construct_place_info(place_info:&str)->u64{
    let mut r:u64=0;
    match place_info {
        "Only_high" => r|=HIGH,
        "Only_low" => r|=LOW,
        "All" => r|=HIGH|LOW,
        "No" =>() ,
        &_ =>()
    }
    r
}
pub fn construct_place_info_from_json(json_path:&str)->Result<Vec<u64>>{
    let mut r =vec![0;super::VEC_LEN];
    let map_path=json_path.to_owned()+"block.json";
    let content = std::fs::read_to_string(map_path)?;
    let j:Value = serde_json::from_str(&content)?;
    // println!("{:?}\n",j);
    for (_key,value) in j["Block"].as_object().unwrap() {
        // println!("key : {:?}, value :{:?}",key,value);
        let info = value["PlaceInfo"].as_str().unwrap();
        let t=construct_place_info(&info);
        let index=value["id"].as_u64().unwrap() as usize;
        // println!("block is {t},index is {index}");
        r[index]=t;
    }
    Ok(r)
}
