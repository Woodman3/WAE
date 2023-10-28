use std::rc::Rc;
use serde::Deserialize;
use crate::utils::config::Config;
use crate::skill::Skill;
use crate::utils::math::{Point, GridRect, Grid};
use crate::unit::enemy;
use crate::unit::enemy::Enemy;
use crate::unit::operator::Operator;
use crate::unit::scope::Scope;

#[derive(Default,Deserialize,Debug)]
struct A{
    v:Option<i32>
}

pub  fn fun(c:&Config){
    // let o:Operator=serde_json::from_value(c.operator["Amiya"].clone()).unwrap();
    let a:A=serde_json::from_value(c.demo["a"].clone()).unwrap();
    println!("{:?}",a);
}
