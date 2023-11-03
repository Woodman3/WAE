use std::rc::Rc;
use serde::Deserialize;
use crate::utils::config::Config;
use crate::skill::Skill;
use crate::utils::math::{Point, GridRect, Grid};
use crate::unit::enemy;
use crate::unit::enemy::Enemy;
use crate::unit::operator::Operator;
use crate::unit::scope::Scope;

#[derive(Default)]
pub struct A{
    v:i32,
    fv:Vec<fn(&mut A)>
}

impl A {
    fn f1(&mut self){
        self.v=1;
    }
    fn f2(&mut self){
        self.v=2;
    }
    fn fun(&mut self){
        for i in 0..self.fv.len(){
            // let f=self.fv[i];
            // f(self);
            self.fv[i](self);
        }
    }
}
pub  fn fun(c:&Config){
    let mut a=A::default();
    // a.fv.push(Box::new(A::f1));
    a.fv.push(A::f1);
    a.fun();
    println!("{}",a.v);
}
