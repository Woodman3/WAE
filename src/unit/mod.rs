pub mod bullet;
pub mod code;
pub(super) mod enemy;
pub(super) mod operator;
pub mod scope;
pub mod skill;

use scope::Scope;
use serde::{Deserialize, Serialize};
use std::cell::{Ref, RefCell};
use std::default;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

use crate::frame::Frame;
use crate::unit::bullet::Bullet;
use crate::unit::skill::effect::{Buff, ChangeClass, ChangeType, DamageType};
use crate::unit::skill::skill_type::AttackType;
use crate::utils::math::Point;
use enemy::Enemy;
use operator::Operator;
use skill::effect::{Effect, FixedDamage};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnitInfo {
    pub(super) damage_type: DamageType,
    pub(super) hp: i64,
    #[serde(skip)]
    pub(super) max_hp: i64,
    pub(super) aspd: f64,
    pub(super) atk: i64,
    pub(super) def: i64,
    pub(super) magic_resist: f64,
    pub(super) attack_time: f64,
    pub(super) block_num: i64,
    pub(super) attack_type: AttackType,
    pub(super) cost: i32,
    pub(super) scope: Scope,
}

// pub trait UnitTrait:Debug{
//     fn get_loc(&self)->Point;
//     fn be_hit(&mut self,b:&Bullet,f:&mut Frame);
//     fn be_damage(&mut self,d:&FixedDamage);
// }

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) enum Unit {
    Enemy(Rc<RefCell<Enemy>>),
    Operator(Rc<RefCell<Operator>>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) enum UnitShared{
    Enemy(Weak<RefCell<Enemy>>),
    Operator(Weak<RefCell<Operator>>),
}

impl UnitInfo {
    pub(crate) fn be_buff(&mut self, b: &Buff) {
        use ChangeClass::*;
        use ChangeType::*;
        let f = match b.change_type {
            DA | LA => std::ops::Add::add,
            DM | LM => std::ops::Mul::mul,
        };
        match b.change_class {
            ASPD => {
                self.aspd = f(self.aspd, b.value);
            }
            ATK => {
                self.atk = f(self.atk as f64, b.value) as i64;
            }
            DEF => {
                self.def = f(self.def as f64, b.value) as i64;
            }
            MaxHP => {
                self.max_hp = f(self.max_hp as f64, b.value) as i64;
            }
            Hp => {
                self.hp = f(self.hp as f64, b.value) as i64;
            }
        }
    }
}

impl Unit {
    pub(crate) fn get_loc(&self) -> Point {
        match &self {
            Unit::Enemy(e) => e.borrow().get_loc(),
            Unit::Operator(o) => o.borrow().get_loc(),
        }
    }
    pub(crate) fn be_hit(&mut self, b: &Bullet, f: &mut Frame) {
        match &self {
            Unit::Enemy(e) => e.borrow_mut().be_hit(b, f),
            Unit::Operator(o) => o.borrow_mut().be_hit(b, f),
        }
    }
    pub(crate) fn be_damage(&mut self, d: &FixedDamage) {
        match &self {
            Unit::Enemy(e) => e.borrow_mut().be_damage(d),
            Unit::Operator(o) => o.borrow_mut().be_damage(d),
        }
    }
    pub(crate) fn be_effect(&mut self, e: &Effect) {
        match &self {
            Unit::Enemy(enemy) => enemy.borrow_mut().be_effect(e),
            Unit::Operator(o) => o.borrow_mut().be_effect(e),
        }
    }
}
