use std::cell::RefCell;
use super::scope::{Scope, Toward};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Weak;
use log::{trace};
use serde::{Deserialize, Serialize};
use serde::ser::Serializer;
use serde_json::Value;
use crate::frame::Frame;
use crate::unit::skill::Skill;
use crate::unit::skill::effect::FixedDamage;
use crate::unit::bullet::Bullet;
use crate::unit::enemy::{EnemyWithPriority,EnemyShared};
use crate::utils::math::{Grid, Point};

mod operator_mission;
#[cfg(test)]
mod test;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub(crate) type OperatorShared = Weak<RefCell<Operator>>;
#[derive(Debug,Clone,Default,Deserialize,Serialize)]
#[serde(default)]
pub struct Operator{
    pub name: String,
    pub info:super::UnitInfo,
    pub stage:super::UnitInfo,
    pub location:Grid,
    pub attack_scope: Scope,
    pub search_scope: Scope,
    pub re_deploy:f32,
    pub(super) toward:Toward,
    pub enemy_find:Vec<EnemyWithPriority>,
    #[serde(serialize_with = "super::enemy::serialize_enemy_shared",skip_deserializing)]
    pub target:EnemyShared,
    #[serde(skip)]
    pub block_vec:Vec<EnemyShared>,
    pub die_code: u32,
    pub skill_ready:Vec<Skill>,
    pub skill_block:Vec<Skill>,
    #[serde(skip)]
    mission_vec:Vec<fn(&mut Operator,&mut Frame)>,
}

impl Operator {
    pub(crate) fn next(&mut self,f:&mut Frame){
        for i in 0..self.mission_vec.len(){
            self.mission_vec[i](self,f);
        }
    }
    pub(super) fn arrange_mission(&mut self){
        self.mission_vec.push(Self::block);
        // self.mission_vec.push(Self::get_target);
        // self.mission_vec.push(Self::attack_mission);
        self.mission_vec.push(Self::skill);
    }
    pub(crate) fn new(v:&Value)->Result<Operator>{
        let mut o:Operator = serde_json::from_value(v.clone())?;
        o.stage=o.info.clone();
        o.search_scope=o.attack_scope.clone();
        o.arrange_mission();
        Ok(o)
    }

    pub(crate) fn deep_clone(&self)->Self{
        Operator{
            enemy_find:Vec::<EnemyWithPriority>::new(),
            target:Weak::new(),
            // block: self.block.clone(),//todo
            ..self.clone()
        }
    }
}

impl Operator {
    pub(super) fn get_loc(&self) -> Point {
        Point::from(self.location)
    }

    pub(super) fn be_hit(&mut self, b: &Bullet, _f: &mut Frame) {
        self.be_damage(&b.damage);
        if self.stage.hp <=0{
            self.die_code=super::code::DIE;
            trace!("an enemy has die!");
            return;
        }
    }


    pub(super) fn be_damage(&mut self, d: &FixedDamage) {
        use super::DamageType::*; 
        match d.damage_type {
            Magical =>{
                let damage=(d.value as f64*(1f64-self.stage.magic_resist)) as i64;
                self.stage.hp -=damage;
            }
            Physical=>{
                let damage=d.value-self.stage.def;
                self.stage.hp -=damage;
            }
            Real=>{
                self.stage.hp -=d.value;
            }
            Heal => {
                self.stage.hp +=d.value;
            },
            None => {},
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

pub(crate) fn serialize_operator_shared<S>(ptr:&OperatorShared, serializer:S) -> std::result::Result<S::Ok,S::Error> where S: Serializer{
    if let Some(e) = ptr.upgrade(){
        let name = e.borrow().name.clone();
        serializer.serialize_str(name.as_str())
    }else{
        serializer.serialize_none()
    }
}
 
