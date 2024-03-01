use std::cell::RefCell;
use std::io;
use std::rc::{Rc, Weak};
use serde::Deserialize;
use crate::frame::Frame;
use crate::utils::config::Config;
use crate::unit::skill::Skill;
use crate::utils::math::{Point, GridRect, Grid};
use crate::unit::{enemy, Unit};
use crate::unit::enemy::Enemy;
use crate::unit::operator::Operator;
use crate::unit::scope::Scope;
// impl DirectSkill {
//     fn schedule(&self,f:Frame){
//
//     }
// }

pub fn fun(){
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{input}");
    match input.trim(){
        "s"=> {
            println!("1");
        }
        "l" =>{
            println!("2");
        }
        &_ => {
            println!("wrong");
        }
    }
}