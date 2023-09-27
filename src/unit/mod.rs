pub mod enemy;
pub mod operator;
pub mod code;
pub mod scope;
pub mod bullet;
mod damage;

use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::frame::Frame;
use crate::unit::bullet::Bullet;
use crate::unit::damage::Damage;
use crate::utils::math::Point;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitInfo {
    damage_type: String,
    health: f64,
    attack_speed: f64,
    damage: f64,
    armor: f64,
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
// #[derive(Debug)]
// pub enum Attacktype {
//     Physical,
//     Magic,
//     Real,
// }


