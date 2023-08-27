use super::scope::{Scope,Toward};
use std::fmt;
use serde_json::Value;

pub struct Operator{
    info:super::UnitInfo,
    location:(u32,u32),
    attack_range:Scope,
    re_deploy:f32,
    toward:Toward,
}

impl Operator {
    pub fn new(v:&Value)->Operator{
        Operator{

            location:(0,0),
            toward:Toward::East,

        }
    }

}