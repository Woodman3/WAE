use super::scope::{Scope,Toward};
use std::fmt;
use serde_json::Value;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug,Clone)]
pub struct Operator{
    info:super::UnitInfo,
    location:(u32,u32),
    attack_range:Scope,
    re_deploy:f32,
    toward:Toward,
}

impl Operator {
    pub fn new(v:&Value)->Result<Operator>{
        let t=serde_json::from_value::<Vec<Vec<f64>>>(v["attack_range"].clone())?;
        let mut t2 = Vec::<((f64,f64),(f64,f64))>::new();
        for ve in t {
            t2.push(((ve[0],ve[1]),(ve[2],ve[3])));
        }
        let attack_range:Scope=Scope::Rect(t2);
        Ok(Operator{
            info: serde_json::from_value::<super::UnitInfo>(v["UnitInfo"].clone())?,
            location:(0,0),
            attack_range,
            re_deploy:serde_json::from_value::<f32>(v["re_deploy"].clone())?,
            toward:Toward::East,
        })
    }

}