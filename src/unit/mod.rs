pub mod enemy;
pub mod operator;
pub mod code;
pub mod scope;
mod bullet;

use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::utils::math::Point;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitInfo {
    name: String,
    attack_type: String,
    health: i64,
    attack_speed: i64,
    damage: i64,
    armor: i64,
    magic_resist: f64,
}

pub trait Unit:Debug{
    fn get_loc(&self)->Point;
}
// #[derive(Debug)]
// pub enum Attacktype {
//     Physical,
//     Magic,
//     Real,
// }


