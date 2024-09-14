use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Deserialize, Debug, Default)]
struct A {
    id: u32,
    #[serde(skip)]
    pub b: Weak<RefCell<B>>,
}
#[derive(Deserialize, Debug, Default)]
struct B {
    id: u32,
    #[serde(skip)]
    pub a: Weak<RefCell<A>>,
}
#[derive(Deserialize, Debug, Default)]
struct C {
    a: Rc<RefCell<A>>,
    b: Rc<RefCell<B>>,
}

impl Serialize for A {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("A", 2)?;
        state.serialize_field("id", &self.id)?;
        let id = self.b.upgrade().unwrap().borrow().id;
        state.serialize_field("b", &id)?;
        state.end()
    }
}
impl Serialize for B {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("B", 2)?;
        state.serialize_field("id", &self.id)?;
        let id = self.a.upgrade().unwrap().borrow().id;
        state.serialize_field("a", &id)?;
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
fn test() {
    let v = serde_json::Value::Null;
    let s = serde_json::to_string(&v).unwrap();
    println!("{s}")
}
