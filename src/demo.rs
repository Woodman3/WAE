use std::cell::{Ref, RefCell};
use std::mem;
use std::rc::Rc;
use serde_json::Value::String;
// use crate::utils::error;
// use crate::utils::error::{ConfigParseError};
use crate::utils::math::{distance_p2p, Grid, Point};
// use crate::unit::scope::Scope;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn fun(){
    // let v = vec![((1,2),(3,4))];
    // let r= Scope(v);
    // let v=vec![Rc::new(RefCell::new(2)),
    //            Rc::new(RefCell::new(3))];
    // let a=Rc::clone(&v[0]);
    // let v2=v.clone();// this is a deep clone,every element in v has copy in new memory
    // so 'a' reference is direct to v,but i need it direct to v2

    let a=Rc::new(RefCell::new(2));
    let p=a.as_ptr();
    println!("{}",p as i64);
    // how to find a position in v?
}

