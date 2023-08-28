use serde::de::Unexpected::Option;
use std::cell::{Cell, RefCell};
use std::ops;
use std::rc::Rc;
use crate::unit::scope::Scope;
// #[derive(Clone)]
// struct A {
//     v: Option<Rc<i32>>,
// }
// struct B {
//     v: A,
//     i: Rc<i32>,
// }
// impl B {
//     pub fn new(i: i32) -> B {
//         B {
//             v: A { v: None },
//             i:Rc::new(i),
//         }
//     }
//     pub fn t(&mut self) {
//         self.v.v = Some(Rc::clone(&self.i));
//     }
// }
#[derive(Debug)]
struct A {
    pub v: i32,
}

impl A {
    fn f(&mut self){
        self.v+=1;
    }
}
pub fn fun() {
    // use std::option::Option;
    // let a=Rc::new(vec![1,2,3]);
    // let b=Some(Rc::clone(&a));
    // // let b:Option<Rc::<Vec::<i32>>>=None;
    // if let Some(ve)=b{
    //     if let Some(v) = ve.get(1){
    //         println!("{v}");
    //     }
    // }
    let mut s = Scope::Rect(vec![((1.2,1.3),(2.3,4.2))]);
    match &mut s {
        Scope::Rect(r) => {
            for (x,y) in r.iter_mut(){
                std::mem::swap(x,y);
            }
            println!("{:?}",r[0]);
        }
        Scope::Circle(_) => {}
    }
}
