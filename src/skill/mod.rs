mod effect;

use eframe::glow::TIMESTAMP;
use effect::Effect;
use serde::Deserialize;
use crate::calculator::PERIOD;

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
    pub duration:f64,
    pub last:f64,
    pub sp_cost:f64,
    pub sp:f64,
    overcharge:bool,
    pub effect:Vec<Effect>
}

impl Skill{
    // fn step(&mut self){
    //     if self.can_charge(){
    //         match self.charge_type {
    //             ChargeType::Auto => {
    //                 self.sp+=PERIOD;
    //             }
    //             _ =>{}
    //         }
    //     }
    //
    // }
    fn ready(&self)->bool{
        self.last!=0.0&&self.sp>=self.sp_cost
    }
    pub fn can_charge(&self)->bool{
        // if self.sp<self.sp_cost{
        //     true
        // }else if self.overcharge{
        //     true
        // }
        // false
        self.sp<self.sp_cost||self.overcharge
    }
}
