use std::cell::{Ref, RefCell};
use std::mem;
use std::rc::Rc;
use serde_json::Value::String;
use crate::utils::error;
use crate::utils::error::{ConfigParseError};
use crate::utils::math::{distance_p2p, Grid};
use crate::unit::scope::Scope;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn fun(){
    // let v = vec![((1,2),(3,4))];
    // let r= Scope(v);
    let mut g:Grid=(1,2).into();
    mem::swap(&mut g.col,&mut g.row);
    g.col=-g.col;
    println!("{:?}",g);
}

