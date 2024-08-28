use crate::calculator::PERIOD;
use crate::frame::{Frame, OperatorRef};
use crate::unit::bullet::Bullet;
use crate::unit::code::DIE;
use crate::unit::operator::OperatorShared;
use crate::unit::skill::effect::FixedDamage;
use crate::utils::math;
use crate::utils::math::{to_target, Point};
use crate::route::Route;
use log::trace;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::rc::{Rc, Weak};

use super::skill::effect::Effect;

#[cfg(test)]
mod test;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub(crate) type EnemyShared = Weak<RefCell<Enemy>>;
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Enemy {
    pub(crate) name: String,
    pub(crate) move_speed: f64,
    pub(crate) info: super::UnitInfo,
    pub(crate) stage: super::UnitInfo,
    pub(crate) location: Point,
    /// -1 mean haven't place
    pub(crate) next_point: Point,
    pub(crate) direction: Point,
    pub(crate) route_stage: usize,
    pub(crate) die_code: u32,
    /// 0 mean haven't die
    pub(crate) route: Rc<Route>,
    #[serde(
        serialize_with = "super::operator::serialize_operator_shared",
        skip_deserializing
    )]
    pub(crate) be_block: OperatorShared,
    pub(crate) id: usize,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnemyWithPriority {
    #[serde(serialize_with = "serialize_enemy_shared", skip_deserializing)]
    pub enemy: EnemyShared,
    pub time_stamp: u64,
}

impl Enemy {
    /// t is 1/fps it mean time interval
    pub fn step(&mut self) {
        let (direction, new) = to_target(self.location, self.next_point, self.move_speed);
        let distance = math::distance_from_segment_to_point(self.location, new, self.next_point);
        if distance <= super::code::MIN_DISTANCE {
            self.route_stage += 1;
            if(self.route_stage<self.route.checkpoints.len()){
                use crate::route::CheckPoint;
                self.next_point = self.route.end;
                for i in self.route_stage..self.route.checkpoints.len(){
                    match self.route.checkpoints[i] {
                        CheckPoint::Move(p) => {self.next_point = p;break},
                        _ => continue,
                    }
                }
            } else if(self.route_stage == self.route.checkpoints.len()){
                self.next_point=self.route.end;
            }else{
                self.die_code = super::code::INTO_END;
            }
        }
        self.direction = direction;
        self.location = new;
    }
    pub(super) fn attack(&mut self, _bv: &mut Vec<Bullet>, o: OperatorRef) {
        if self.stage.attack_time > 0.0 {
            self.stage.attack_time -= PERIOD;
        } else {
            use super::AttackType::*;
            match self.stage.attack_type {
                Melee => {
                    let d = FixedDamage {
                        value: self.stage.atk,
                        damage_type: self.stage.damage_type.clone(),
                    };
                    o.borrow_mut().be_damage(&d);
                    // self.target.upgrade().unwrap().borrow_mut().be_damage(&d);
                }
                Ranged => {
                    todo!("ranged enemy");
                    //todo: ranged enemy
                    // bv.push(Bullet::new(
                    //     self.target.upgrade().unwrap(),
                    //     Point::from(self.location),
                    //     2f64,
                    //     self.stage.damage_type.clone(),
                    //     self.stage.damage,
                    // ));
                }
                _ => {
                    todo!("unknown attack type of enemy");
                }
            }
            self.stage.attack_time = self.info.attack_time;
        }
    }
    pub(crate) fn next(&mut self, f: &mut Frame) {
        if let Some(o) = self.be_block.upgrade() {
            self.attack(&mut f.bullet_set, o);
        } else {
            self.step();
        }
        if self.stage.hp <= 0 {
            self.die_code = DIE;
        }
    }
    pub fn new(v: &Value) -> Result<Enemy> {
        let mut e: Self = serde_json::from_value(v.clone())?;
        e.stage = e.info.clone();
        e.route_stage = 1;
        Ok(e)
    }
}

impl fmt::Display for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
            component_x:{} component_y:{}\n\
            health:{} \n\
            ",
            self.direction.x, self.direction.y, self.stage.hp,
        )
    }
}

impl Enemy {
    pub(super) fn get_loc(&self) -> Point {
        self.location
    }
    pub(super) fn be_hit(&mut self, b: &Bullet, _f: &mut Frame) {
        self.be_damage(&b.damage);
    }
    pub(super) fn be_damage(&mut self, d: &FixedDamage) {
        use super::DamageType::*;
        match d.damage_type {
            Magical => {
                let damage = (d.value as f64 * (1.0 - self.stage.magic_resist)) as i64;
                self.stage.hp -= damage;
            }
            Physical => {
                let damage = d.value - self.stage.def;
                self.stage.hp -= damage;
            }
            Real => {
                self.stage.hp -= d.value;
            }
            Heal => {
                self.stage.hp += d.value;
            }
            None => {} // _ => {
                       //     warn!("unknown attack type of bullet ,bullet has been departure");
                       //     return
                       // }
        }
        if self.stage.hp <= 0 {
            self.die_code = super::code::DIE;
            trace!("an enemy has die!");
            return;
        }
    }

    pub(super) fn be_effect(&mut self, e: &Effect) {
        todo!()
    }
}

impl PartialEq<Self> for Enemy {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Enemy {}

impl Eq for EnemyWithPriority {}

impl PartialEq<Self> for EnemyWithPriority {
    fn eq(&self, other: &Self) -> bool {
        self.time_stamp == other.time_stamp
    }
}

impl PartialOrd<Self> for EnemyWithPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.time_stamp.partial_cmp(&other.time_stamp)
    }
}

impl Ord for EnemyWithPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_stamp.cmp(&other.time_stamp)
    }
}

pub(crate) fn serialize_enemy_shared<S>(
    ptr: &EnemyShared,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // let id = ptr.upgrade().unwrap().borrow().id as u64;
    if let Some(e) = ptr.upgrade() {
        let id = e.borrow().id as u64;
        serializer.serialize_u64(id)
    } else {
        serializer.serialize_none()
    }
}
