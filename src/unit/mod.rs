pub(super) mod enemy;
pub mod code;
pub mod scope;
pub mod bullet;
pub mod skill;
pub(super) mod operator;

use std::cell::{Ref, RefCell};
use std::fmt::Debug;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use serde::ser::{Serializer};
use crate::frame::Frame;
use crate::unit::bullet::Bullet;
use skill::effect::FixedDamage;
use crate::unit::skill::effect::{ChangeClass, ChangeType, Buff, DamageType};
use crate::unit::skill::skill_type::AttackType;
use operator::Operator;
use enemy::Enemy;
use crate::utils::math::Point;

#[derive(Debug, Clone,Default, Deserialize,Serialize)]
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


// pub trait UnitTrait:Debug{
//     fn get_loc(&self)->Point;
//     fn be_hit(&mut self,b:&Bullet,f:&mut Frame);
//     fn be_damage(&mut self,d:&FixedDamage);
// }

#[derive(Clone,Debug,Deserialize,Serialize)]
pub(super) enum Unit {
    Enemy(Rc<RefCell<Enemy>>),
    Operator(Rc<RefCell<Operator>>),
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

impl Unit{
    pub(super) fn get_loc(&self)->Point{
        match &self {
            Unit::Enemy(e) => {
                e.borrow().get_loc()
            },
            Unit::Operator(o) => {
                o.borrow().get_loc()
            },
        }
    }
    pub(super) fn be_hit(&mut self,b:&Bullet,f:&mut Frame){
        match &self {
            Unit::Enemy(e) => {
                e.borrow_mut().be_hit(b,f)
            },
            Unit::Operator(o) => {
                o.borrow_mut().be_hit(b,f)
            },
        }
    }
    pub(super) fn be_damage(&mut self,d:&FixedDamage){
        match &self {
            Unit::Enemy(e) => {
                e.borrow_mut().be_damage(d)
            },
            Unit::Operator(o) => {
                o.borrow_mut().be_damage(d)
            },
        }
    }
}

