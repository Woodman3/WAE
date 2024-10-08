use crate::calculator::PERIOD;
use crate::event::Event;
use crate::frame::{Frame, OperatorRef};
use crate::route::Route;
use crate::unit::bullet::Bullet;
use crate::unit::operator::OperatorShared;
use crate::unit::skill::effect::FixedDamage;
use crate::utils::math::Point;
use log::trace;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::rc::{Rc, Weak};

use super::skill::effect::{self, Damage, Effect};
use super::skill::skill_schedule::SkillSchedule;
use super::skill::skill_type::{ScheduleType, TriggerType};
use super::skill::{Skill, SkillEntity, ToOperatorSkill};

mod enemy_mission;
#[cfg(test)]
mod test;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub(crate) type EnemyShared = Weak<RefCell<Enemy>>;
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub(crate) struct Enemy {
    pub(crate) name: String,
    pub(crate) move_speed: f64,
    pub(crate) info: super::UnitInfo,
    pub(crate) stage: super::UnitInfo,
    pub(crate) location: Point,
    pub(crate) direction: Point,
    pub(crate) route_stage: usize,
    pub(crate) die_code: u32,
    /// 0 mean haven't die
    pub(crate) route: Route,
    #[serde(
        serialize_with = "super::operator::serialize_operator_shared",
        skip_deserializing
    )]
    pub(crate) be_block: OperatorShared,
    pub(crate) id: usize,
    #[serde(skip)]
    pub(crate) mission_vec: Vec<fn(&mut Enemy, &mut Frame)>,
    pub(crate) skills: SkillSchedule,
}

impl Enemy {
    pub(crate) fn init(&mut self) {
        self.arrange_mission();
        let default_skill = self.generate_default_attack_skill();
        self.skills.skill_block.push(default_skill);
    }
    pub(crate) fn generate_default_attack_skill(&mut self) -> Skill {
        let d = effect::Damage {
            value: self.stage.atk,
            change: Option::None,
            damage_type: self.stage.damage_type,
        };
        let skill_entity = SkillEntity::ToOperatorSkill(ToOperatorSkill {
            target: Vec::new(),
            target_num: 1,
            effect: effect::Effect::Damage(d),
            attack_type: self.stage.attack_type,
            search_scope: Option::from(self.stage.scope.clone()),
        });
        Skill {
            trigger_type: TriggerType::Auto,
            schedule_type: ScheduleType::Immediately,
            duration: self.stage.attack_time,
            last: self.stage.attack_time,
            skill_entity,
            ..Default::default()
        }
    }
    pub(crate) fn next(&mut self, f: &mut Frame) {
        for i in 0..self.mission_vec.len() {
            self.mission_vec[i](self, f);
        }
        if self.stage.hp <= 0 {
            f.events.push(Event::EnemyDieEvent(self.id));
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
        self.be_effect(&b.effect);
    }
    pub(super) fn be_damage(&mut self, d: &Damage) {
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
        // maybe it should be check in next turn?
        // if self.stage.hp <= 0 {
        //     f.events.push(Event::EnemyDieEvent(self.id));
        //     return;
        // }
    }

    pub(super) fn be_effect(&mut self, e: &Effect) {
        match e {
            Effect::Buff(b) => {
                self.stage.be_buff(b);
            }
            Effect::Damage(d) => {
                self.be_damage(&d);
            }
            _ => {}
        }
    }
}

impl PartialEq<Self> for Enemy {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Enemy {}

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
