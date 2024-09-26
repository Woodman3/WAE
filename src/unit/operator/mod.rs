use super::scope::Toward;
use super::skill::effect::{self, Damage, Effect};
use super::skill::skill_schedule::SkillSchedule;
use super::skill::skill_type::{ScheduleType, TriggerType};
use super::skill::{SkillEntity, ToEnemySkill};
use crate::event::Event;
use crate::frame::Frame;
use crate::unit::bullet::Bullet;
use crate::unit::enemy::EnemyShared;
use crate::unit::skill::Skill;
use crate::utils::math::{Grid, Point};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Weak;

mod operator_mission;
#[cfg(test)]
mod test;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub(crate) type OperatorShared = Weak<RefCell<Operator>>;
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub(crate) struct Operator {
    pub(crate) name: String,
    pub(crate) info: super::UnitInfo,
    pub(crate) stage: super::UnitInfo,
    pub(crate) location: Grid,
    pub(crate) re_deploy: f32,
    pub(crate) toward: Toward,
    #[serde(
        serialize_with = "super::enemy::serialize_enemy_shared",
        skip_deserializing
    )]
    pub(crate) target: EnemyShared,
    #[serde(skip)]
    pub(crate) block_vec: Vec<EnemyShared>,
    pub(crate) die_code: u32,
    #[serde(skip)]
    mission_vec: Vec<fn(&mut Operator, &mut Frame)>,
    pub(crate) skills: SkillSchedule,
}

impl Operator {
    pub(crate) fn next(&mut self, f: &mut Frame) {
        for i in 0..self.mission_vec.len() {
            self.mission_vec[i](self, f);
        }
    }

    pub(crate) fn init(&mut self) {
        self.arrange_mission();
        let default_skill = self.generate_default_attack_skill();
        self.skills.skill_block.push(default_skill);
    }

    pub(crate) fn arrange_mission(&mut self) {
        self.mission_vec.push(Self::block);
        // self.mission_vec.push(Self::get_target);
        // self.mission_vec.push(Self::attack_mission);
        self.mission_vec.push(Self::skill_mission);
    }
    pub(crate) fn new(v: &Value) -> Result<Operator> {
        let mut o: Operator = serde_json::from_value(v.clone())?;
        o.stage = o.info.clone();
        o.arrange_mission();
        Ok(o)
    }

    pub(crate) fn deep_clone(&self) -> Self {
        Operator {
            target: Weak::new(),
            // block: self.block.clone(),//todo
            ..self.clone()
        }
    }
    pub(super) fn get_loc(&self) -> Point {
        Point::from(self.location)
    }

    pub(super) fn be_hit(&mut self, b: &Bullet, f: &mut Frame) {
        self.be_effect(&b.effect);
        if self.stage.hp <= 0 {
            f.events
                .push(Event::OperatorRetreatEvent(self.name.clone()));
            return;
        }
    }

    pub(super) fn be_damage(&mut self, d: &Damage) {
        use super::DamageType::*;
        match d.damage_type {
            Magical => {
                let damage = (d.value as f64 * (1f64 - self.stage.magic_resist)) as i64;
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
            None => {}
        }
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
    pub(crate) fn generate_default_attack_skill(&mut self) -> Skill {
        let d = effect::Damage {
            value: self.stage.atk,
            change: Option::None,
            damage_type: self.stage.damage_type,
        };
        let skill_entity = SkillEntity::ToEnemySkill(ToEnemySkill {
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
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "\
        name:{}\n\
        loc:{}\n\
        block_num:{}\n\
        block_vec_len:{}\n\
        skills:{}
        ",
            self.name,
            self.location,
            self.stage.block_num,
            self.block_vec.len(),
            self.skills,
        )?;
        Ok(())
    }
}

pub(crate) fn serialize_operator_shared<S>(
    ptr: &OperatorShared,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(e) = ptr.upgrade() {
        let name = e.borrow().name.clone();
        serializer.serialize_str(name.as_str())
    } else {
        serializer.serialize_none()
    }
}
