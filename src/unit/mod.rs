pub mod enemy;
pub mod operator;
pub mod code;
pub mod scope;
pub mod bullet;

use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::frame::Frame;
use crate::unit::bullet::Bullet;
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
// #[derive(Debug,Clone)]
// pub struct UnitStage{
//     damage_type: String,
//     health: f64,
//     attack_speed: f64,
//     damage: f64,
//     armor: f64,
//     magic_resist: f64,
//     attack_time:f64,
//     block_num:u32,
// }

pub trait Unit:Debug{
    fn get_loc(&self)->Point;
    fn be_hit(&mut self,b:&Bullet,f:&mut Frame);
}

// impl From<UnitInfo> for UnitStage {
//     fn from(value: UnitInfo) -> Self {
//         UnitStage{
//             attack_time:value.attack_time,
//             health:value.health,
//             attack_speed:value.attack_speed,
//             damage:value.damage,
//             armor:value.armor,
//             magic_resist:value.magic_resist,
//             damage_type:value.damage_type,
//             block_num:value.block_num,
//         }
//     }
// }
// #[derive(Debug)]
// pub enum Attacktype {
//     Physical,
//     Magic,
//     Real,
// }


