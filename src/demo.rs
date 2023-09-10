use std::cell::{Ref, RefCell};
use std::rc::Rc;
use serde_json::Value::String;
use crate::utils::error;
use crate::utils::error::{ConfigParseError};
use crate::utils::math::distance_p2p;
use crate::unit::scope;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn fun(){
    let mut r=scope::Scope::Rect(vec![((-1,0),(1,4))]);
    r.apply_loc((1,2),4,5);
    println!("{:?}",r);
}
