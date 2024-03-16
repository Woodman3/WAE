use super::*;
use serde_json::{to_string_pretty};
#[test]
fn test_serialize(){
    let o = Operator::default();
    let json = to_string_pretty(&o).unwrap();
    println!("{}",json);
}