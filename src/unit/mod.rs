pub(super) mod enemy;
pub mod code;
pub mod scope;
pub mod bullet;
pub mod skill;
pub(super) mod operator;

use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::frame::Frame;
use crate::unit::bullet::Bullet;
use skill::effect::FixedDamage;
use crate::unit::skill::effect::{ChangeClass, ChangeType, Buff, DamageType};
use crate::unit::skill::skill_type::AttackType;
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
    attack_type:AttackType,
}

pub trait Unit:Debug{
    fn get_loc(&self)->Point;
    fn be_hit(&mut self,b:&Bullet,f:&mut Frame);
    fn be_damage(&mut self,d:&FixedDamage);
}

impl UnitInfo {
    pub fn be_buff(&mut self,b: Buff){
        use ChangeType::*;
        use ChangeClass::*;
        let f=match b.change_type {
            DA|LA => {std::ops::Add::add}
            DM|LM => {std::ops::Mul::mul}
        };
        match b.change_class {
            ASPD => {
                self.aspd=f(self.aspd,b.value);
            }
            ATK => {
                self.atk=f(self.atk,b.value);
            }
            DEF => {
                self.def=f(self.def,b.value);
            }
            MaxHP => {
                self.max_hp=f(self.max_hp,b.value);
            }
            Hp => {
                self.hp=f(self.hp,b.value);
            }
        }
    }
}


