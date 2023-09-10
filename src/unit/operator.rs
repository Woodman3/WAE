use super::scope::{Scope,Toward};
use std::fmt;
use serde_json::Value;
use crate::map::Map;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug,Clone)]
pub struct Operator{
    info:super::UnitInfo,
    pub location:(u32,u32),
    pub attack_scope:Scope,
    pub search_scope:Scope,
    pub re_deploy:f32,
    pub toward:Toward,
}

impl Operator {
    pub fn new(v:&Value)->Result<Operator>{
        let t=serde_json::from_value::<Vec<Vec<i32>>>(v["attack_range"].clone())?;
        let mut t2 = Vec::<((i32,i32),(i32,i32))>::new();
        for ve in t {
            t2.push(((ve[0],ve[1]),(ve[2],ve[3])));
        }
        let attack_scope:Scope=Scope::Rect(t2);
        let search_scope=attack_scope.clone();
        Ok(Operator{
            info: serde_json::from_value::<super::UnitInfo>(v["UnitInfo"].clone())?,
            location:(0,0),
            attack_scope,
            search_scope,
            re_deploy:serde_json::from_value::<f32>(v["re_deploy"].clone())?,
            toward:Toward::East,
        })
    }
    fn init_search_scope(&mut self){
        let s=self.attack_scope.apply_toward(&self.toward);

    }
    fn search(&self,m:Map){

    }
}