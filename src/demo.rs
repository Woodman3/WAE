use std::rc::Rc;
use serde::Deserialize;
use crate::utils::config::Config;
use crate::unit::skill::Skill;
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
fn mymul<T>(x:T,y:T)->T
where T:std::ops::Mul<Output = T>
{
    x*y
}
pub fn fun(c:&Config){
    // let f=std::ops::Mul::mul;
    let mut a=3.2;
    let mut b :i32=2;
    a=mymul(a,3.2);
    b=mymul(b,2);
    println!("{},{}",a,b);
}
