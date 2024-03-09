use std::{cell::RefCell, ops::Deref, rc::{Rc, Weak} };
use serde::{Deserialize, Serialize};
use serde::ser::{SerializeStruct,Serializer};
use serde_json::{from_value, json, to_string};

#[derive(Deserialize,Debug,Default)]
struct A{
    id:u32,
    #[serde(skip)]
    pub b:Weak<RefCell<B>>
}
#[derive(Deserialize,Debug,Default)]
struct B{
    id:u32,
    #[serde(skip)]
    pub a:Weak<RefCell<A>>
}
#[derive(Deserialize,Debug,Default)]
struct C{
    a:Rc<RefCell<A>>,
    b:Rc<RefCell<B>>,
    _enityA:Vec<A>,
    _enityB:Vec<B>
}

impl Serialize for A{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut state=serializer.serialize_struct("A", 2)?;
            state.serialize_field("id",&self.id)?;
            let id= self.b.upgrade().unwrap().borrow().id;
            state.serialize_field("b",&id)?;
            state.end()
    }
}
impl Serialize for B{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
            let mut state=serializer.serialize_struct("B", 2)?;
            state.serialize_field("id",&self.id)?;
            let id= self.a.upgrade().unwrap().borrow().id;
            state.serialize_field("a",&id)?;
            state.end()
    }
}
// impl Serialize for C{
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer {
//             let mut state = serializer.serialize_struct("C", 2)?;
//             state.serialize_field("a", self.a.borrow().deref())?;
//             state.serialize_field("b", self.b.borrow().deref())?;
//             state.end()
//     }
// }
// impl Deserialize for C{
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de> {
//             #[derive(Deserialize)]
//             enum Field {A,B};

//     }
// }

#[test]
fn test(){
    // let mut ra= Rc::new(RefCell::new(A{id:0,b:Weak::new()})); 
    // let mut rb = Rc::new(RefCell::new(B{id:0,a:Weak::new()}));
    // rb.borrow_mut().a=Rc::downgrade(&ra);
    // ra.borrow_mut().b=Rc::downgrade(&rb);
    // let c=C{a:ra,b:rb};
    // let j=to_string(&c).unwrap();
    // println!("{j}");
    // let j = json!(
    //     {
    //         "a":{"id":1,"b":2},
    //         "b":{"id":2,"a":1}
    //     }
    // );
    // let c:C=from_value(j).unwrap();
    let c=C::default();
    println!("{:?}",c);
}