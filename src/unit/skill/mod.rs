pub mod effect;

use std::collections::HashMap;
use eframe::glow::TIMESTAMP;
use log::warn;
use effect::Buff;
use serde::Deserialize;
use serde_json::Value;
use crate::calculator::PERIOD;
use crate::frame::OperatorRef;
use crate::unit::operator::Operator;
use crate::unit::skill::effect::Effect;
use crate::utils::config::Config;

#[derive(Clone,Deserialize,Debug,Default,PartialEq)]
pub enum ChargeType {
    Auto,
    Attack,
    BeHit,
    Passive,
    #[default]
    None
}
#[derive(Clone,Deserialize,Debug,Default)]
pub enum TriggerType{
    Auto,
    Manual,
    Passive,
    #[default]
    None
}
#[derive(Clone,Deserialize,Debug,Default)]
pub struct Skill{
    pub charge_type: ChargeType,
    pub trigger_type:TriggerType,
    pub duration:f64, ///skill time
    #[serde(skip)]
    pub last:f64,///if in skill ,it show time remain,or is 0
    pub sp_cost:f64,
    pub sp:f64,
    overcharge:bool,
    pub effect:Vec<Effect>
}
pub fn config_skill(c:&Config,os:&HashMap<String, OperatorRef>){
    for (key,skill) in c.doctor["skill"].as_object().unwrap(){
        if let Some(value) = c.skill.get(key).unwrap().get(skill.as_str().unwrap()){
            if let Some(o) =os.get(key){
                o.borrow_mut().skill = Some(serde_json::from_value(value.clone()).unwrap());
            } else{
                warn!("unknown operator name in skill config!")
            }
        } else{
            warn!("unknown skill name in skill config!,skill name:{}",skill)
        }
    }
}
impl Skill{
    pub fn ready(&self)->bool{
        self.last!=0.0&&self.sp>=self.sp_cost
    }
    pub fn can_charge(&self)->bool{ self.sp<self.sp_cost||self.overcharge }
    pub fn charge(&mut self,value:f64){
        if self.can_charge(){
            self.sp+=value
        }
    }
}
