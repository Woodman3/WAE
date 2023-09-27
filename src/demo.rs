use std::cell::{Ref, RefCell};
use std::mem;
use std::rc::Rc;
use serde_json::Value::String;
use crate::sub2d;
// use crate::utils::error;
// use crate::utils::error::{ConfigParseError};
use crate::utils::math::{distance_p2p, Grid, Point};
// use crate::unit::scope::Scope;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Clone)]
struct B{
    v:i32
}
impl B{
    fn f(&self,a:&mut A){}
}
struct A{
    v:Vec<B>,
}

pub fn fun(){
    // let mut a=A{v:vec![B{v:1},B{v:2}]};
    // let v:Vec<B> =a.v.iter().cloned().collect();
    // for b in v.iter(){
    //     b.f(&mut a);
    // }
    let a=Rc::new(2);
    let b=Rc::downgrade(&a);
    if let Some(t)=b.upgrade(){
        println!("{t}");
    }
}

