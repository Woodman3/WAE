use std::cell::RefCell;
use super::scope::{Scope, Toward};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ptr::write;
use std::rc::{Rc, Weak};
use log::{error, info, trace, warn};
use serde::Deserialize;
use serde_json::Value;
use crate::calculator::PERIOD;
use crate::frame::Frame;
use crate::map::Map;
use crate::unit::skill::Skill;
use crate::unit::skill::effect::FixedDamage;
use crate::unit::skill::skill_type::{AttackType,ChargeType};
use crate::unit::bullet::Bullet;
use crate::unit::enemy::{Enemy, EnemyWithPriority};
use crate::unit::Unit;
use crate::utils::math::{Grid, GridRect, Point};

mod operator_mission;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug,Clone,Default,Deserialize)]
pub struct Operator{
    pub name: String,
    pub info:super::UnitInfo,
    #[serde(skip)]
    pub stage:super::UnitInfo,
    #[serde(skip)]
    pub location:Grid,
    pub attack_scope: Scope,
    #[serde(skip)]
    pub search_scope: Scope,
    pub re_deploy:f32,
    #[serde(skip)]
    pub toward:Toward,
    #[serde(skip)]
    pub enemy_find:Vec<EnemyWithPriority>,
    #[serde(skip)]
    pub target:Weak<RefCell<Enemy>>,
    #[serde(skip)]
    pub block_vec:Vec<Weak<RefCell<Enemy>>>,
    #[serde(skip)]
    pub die_code: u32,
    #[serde(skip)]
    pub skill_ready:Vec<Skill>,
    #[serde(skip)]
    pub skill_block:Vec<Skill>,
    #[serde(skip)]
    mission_vec:Vec<fn(&mut Operator,&mut Frame)>,
}

impl Operator {
    pub fn next(&mut self,f:&mut Frame){
        for i in 0..self.mission_vec.len(){
            self.mission_vec[i](self,f);
        }
    }
    pub fn arrange_mission(&mut self){
        self.mission_vec.push(Self::block);
        // self.mission_vec.push(Self::get_target);
        // self.mission_vec.push(Self::attack_mission);
        self.mission_vec.push(Self::skill);
    }
    pub fn new(v:&Value)->Result<Operator>{
        let mut o:Operator = serde_json::from_value(v.clone())?;
        o.stage=o.info.clone();
        o.search_scope=o.attack_scope.clone();
        o.arrange_mission();
        Ok(o)
    }

    pub fn deep_clone(&self)->Self{
        Operator{
            enemy_find:Vec::<EnemyWithPriority>::new(),
            target:Weak::new(),
            // block: self.block.clone(),//todo
            ..self.clone()
        }
    }
}

impl Unit for Operator {
    fn get_loc(&self) -> Point {
        Point::from(self.location)
    }

    fn be_hit(&mut self, b: &Bullet, f: &mut Frame) {
        self.be_damage(&b.damage);
        if self.stage.hp <=0f64{
            self.die_code=super::code::DIE;
            trace!("an enemy has die!");
            return;
        }
    }


    fn be_damage(&mut self, d: &FixedDamage) {
        use super::skill::effect::DamageType::*;
        match d.damage_type {
            Magic =>{
                let damage=d.value*(1f64-self.stage.magic_resist);
                self.stage.hp -=damage;
            }
            Physical=>{
                let damage=d.value-self.stage.def;
                self.stage.hp -=damage;
            }
            Real=>{
                self.stage.hp -=d.value;
            }
            _ => {
                warn!("unknown attack type of bullet ,bullet has been departure");
                return
            }
        }
    }
}

impl Display for Operator{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"\
        block_num:{}\n\
        block_vec_len:{}\n\
        ",
               self.stage.block_num,
               self.block_vec.len(),
        )?;
        for i in 0..self.skill_ready.len(){
            write!(f,"{i} of ready skill : {}",self.skill_ready[i])?;
        }
        for i in 0..self.skill_block.len(){
            write!(f,"{i} of block skill : {}",self.skill_block[i])?;
        }
        write!(f,"")
    }
}
