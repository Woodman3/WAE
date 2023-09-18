use std::cell::{Ref, RefCell};
use std::mem;
use std::rc::Rc;
use serde_json::Value::String;
// use crate::utils::error;
// use crate::utils::error::{ConfigParseError};
use crate::utils::math::{distance_p2p, Grid, Point};
// use crate::unit::scope::Scope;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
trait T{
    fn t(&self);
}
struct A(i32);

impl T for A {
    fn t(&self) {
        println!("{}",self.0);
    }
}
pub fn fun(){
    let mut v=vec![1,2,3,4];
    let v2:Vec<i32>=v.iter().filter(|&x| x%2!=0).cloned().collect();
    v.retain(|&x| x%2==0);
    print!("{:?}",v2);

}

