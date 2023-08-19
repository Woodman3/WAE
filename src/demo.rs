use std::rc::Rc;
use std::cell::{Cell,RefCell};
use serde::de::Unexpected::Option;

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
pub fn fun() {
    use std::option::Option;
    let a=Rc::new(vec![1,2,3]);
    let b=Some(Rc::clone(&a));
    // let b:Option<Rc::<Vec::<i32>>>=None;
    if let Some(ve)=b{
        if let Some(v) = ve.get(1){
            println!("{v}");
        }
    }

}
