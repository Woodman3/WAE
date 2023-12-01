pub mod effect;
pub mod skill_type;
pub mod skill_schedule;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;
use eframe::glow::TIMESTAMP;
use log::warn;
use effect::Buff;
use serde::Deserialize;
use serde_json::Value;
use crate::calculator::PERIOD;
use crate::frame::OperatorRef;
use crate::unit::enemy::Enemy;
use crate::unit::operator::Operator;
use crate::unit::skill::effect::Effect;
use crate::utils::config::Config;
use skill_type::*;

#[derive(Clone,Deserialize,Debug,Default)]
pub struct Skill{
    pub charge_type: ChargeType,
    pub trigger_type:TriggerType,
    pub schedule_type:ScheduleType,
    pub duration:f64, ///skill time
    #[serde(skip)]
    pub last:f64,///if in skill ,it show time remain,or is 0
    pub sp_cost:f64,
    pub sp:f64,
    overcharge:bool,
    pub effect:Vec<Effect>
}
#[derive(Default)]
pub struct AttackSkill {
    pub target:Weak<RefCell<Enemy>>,
    pub target_effect:Effect,
    pub self_effect:Effect,
    pub attack_type:AttackType,
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
