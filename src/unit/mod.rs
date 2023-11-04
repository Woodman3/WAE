pub mod enemy;
pub mod operator;
pub mod code;
pub mod scope;
pub mod bullet;
mod operator_mission;
pub mod skill;

use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::frame::Frame;
use crate::unit::bullet::Bullet;
use skill::effect::Damage;
use crate::unit::skill::effect::{ChangeClass, ChangeType, Buff, DamageType};
use crate::utils::math::Point;

#[derive(Debug, Clone,Default, Deserialize)]
pub struct UnitInfo {
    damage_type: DamageType,
    hp: f64,
    #[serde(skip)]
    max_hp:f64,
    aspd: f64,
    atk: f64,
    def: f64,
    magic_resist: f64,
    attack_time:f64,
    block_num:u32,
    attack_type:String,
}

pub trait Unit:Debug{
    fn get_loc(&self)->Point;
    fn be_hit(&mut self,b:&Bullet,f:&mut Frame);
    fn be_damage(&mut self,d:&Damage);
}

impl UnitInfo {
    pub fn be_buff(&mut self,b: Buff){
        let f=match b.change_type {
            ChangeType::Absolute => {std::ops::Add::add}
            ChangeType::Relative => {std::ops::Mul::mul}
        };
        match b.change_class {
            ChangeClass::ASPD => {
                self.aspd=f(self.aspd,b.value);
            }
            ChangeClass::ATK => {
                self.atk=f(self.atk,b.value);
            }
            ChangeClass::DEF => {
                self.def=f(self.def,b.value);
            }
            ChangeClass::MaxHP => {
                self.max_hp=f(self.max_hp,b.value);
            }
            ChangeClass::Hp => {
                self.hp=f(self.hp,b.value);
            }
        }
    }
}


