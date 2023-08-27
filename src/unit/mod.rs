pub mod enemy;
pub mod operator;
pub mod code;
pub mod scope;

use serde::{Deserialize, Serialize};
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
// #[derive(Debug)]
// pub enum Attacktype {
//     Physical,
//     Magic,
//     Real,
// }

// #[derive(Debug)]
// pub struct Enemy {
//     // info:Info,
//     pub x:f64,
//     pub y:f64,
// }
// impl Enemy {
//     pub fn get_position(&self)->f64 {
//         self.y
//     }
// }
// #[derive(Debug)]
// struct Operator {
//     name:String,
//     info:EnityInfo,
//     x:u64,
//     y:u64,
// }
// pub trait Enity {
//     fn get_position(&self)->f64;
// }
// impl Enity for Enemy{
//     fn get_position(&self)->f64
//     {
//         self.x
//     }
// }
// impl Enity for Operator{
//     fn get_position(&self)->f64
//     {
//         self.x
//     }
// }
//
