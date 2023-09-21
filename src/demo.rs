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
    let a=(1.0,1.0);
    let b=(3.0,4.0);
    let c =crate::mul2d!(a,b);
    let A:Point=a.into();
    let B:Point=b.into();
    let C=A*B;
    print!("{:?} {:?}",c,C);

}

// let mut v=vec![1,2,3,4];
// let v2:Vec<i32>=v.iter().filter(|&x| x%2!=0).cloned().collect();
// v.retain(|&x| x%2==0);
// print!("{:?}",v2);
