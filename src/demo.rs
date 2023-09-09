use std::cell::{Ref, RefCell};
use std::rc::Rc;
use serde_json::Value::String;
use crate::utils::error;
use crate::utils::error::{ConfigParseError};
use crate::utils::math::distance_p2p;

struct A{
    v:i32,
}

impl A {
    pub fn t(&mut self) {self.v+=1;}
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn fun(){
    // let mut a=Rc::new(RefCell::new(3));
    // let a=Rc::new(RefCell::new(3));
    // let b=Rc::new(RefCell::clone(&a));
    // *a.borrow_mut()=4;
    // println!("{}",b.borrow());
}
