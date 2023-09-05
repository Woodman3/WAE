use std::rc::Rc;
use serde_json::Value::String;
use crate::utils::error;
use crate::utils::error::{ConfigParseError};

struct A{
    v:i32,
}

impl A {
    pub fn t(&mut self) {self.v+=1;}
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn fun(){
    // use std::option::Option;
    // let a=Rc::new(vec![1,2,3]);
    // let b=Some(Rc::clone(&a));
    // // let b:Option<Rc::<Vec::<i32>>>=None;
    // if let Some(ve)=b{
    //     if let Some(v) = ve.get(1){
    //         println!("{v}");
    //     }
    // }
    let mut a1=A{v:2};
    let a2=Rc::new(&a1);
    println!("{},{}",a1.v,a2.v);
}
