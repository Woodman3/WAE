use std::env;
use std::fs;
use serde_json::{Result,Value};
fn main() {
    // let arg: Vec<String> = env::args().collect();
    // dbg!(arg);
    let file_path="/home/archer/workspace/BEC/map.json";
    let content = fs::read_to_string(file_path).expect(&format!("can't get the path {file_path}"));
    let data:Value=serde_json::from_str(&content).expect(&format!("can't read json,file content :\n {content} \n"));
    let layout = data["layout"].clone();
    println!("{layout},\n{},\n{}",layout[1],layout[1][2]);
}
