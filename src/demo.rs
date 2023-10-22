use std::rc::Rc;
use serde::Deserialize;
use crate::utils::config::Config;
use crate::skill::Skill;
use crate::utils::math::Point;
use crate::unit::enemy;
use crate::unit::enemy::Enemy;

#[derive(Default,Deserialize,Debug)]
#[serde(default)]
struct A{
    // v:Option<i32>,//it work
    #[serde(skip)]
        v: Option<Rc<i32>>,//it can't work
    v2:i32
}
pub  fn fun(c:&Config){
    // let t = enemy::Enemy::new(&c.enemy["Dog"]).unwrap();
    let t:Enemy=serde_json::from_value(c.enemy["Dog"].clone()).unwrap();
    println!("{:?}",t);
}
