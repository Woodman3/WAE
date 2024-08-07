pub mod effect;
mod skill_fn;
pub mod skill_schedule;
pub mod skill_type;
use crate::frame::OperatorRef;
use crate::unit::enemy::Enemy;
use crate::unit::scope::Scope;
use crate::unit::skill::effect::Effect;
use crate::utils::config::Config;
use log::warn;
use serde::{Deserialize, Serialize};
use skill_type::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(default)]
pub struct Skill {
    pub charge_type: ChargeType,
    pub trigger_type: TriggerType,
    pub schedule_type: ScheduleType,
    pub duration: f64,
    ///skill time
    pub last: f64,
    ///if in skill ,it show time remain,or is 0
    pub sp_cost: f64,
    pub sp: f64,
    pub(crate) overcharge: bool,
    pub(crate) skill_entity: SkillEntity,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
pub(crate) struct ToEnemySkill {
    #[serde(skip)]
    pub(crate) target: Vec<Weak<RefCell<Enemy>>>,
    pub(self) target_num: usize,
    pub(crate) effect: Effect,
    pub(self) attack_type: AttackType,
    pub(self) search_scope: Option<Scope>,
}

pub(crate) struct NotDirectSkill {}

#[derive(Default, Deserialize, Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub(crate) enum SkillEntity {
    ToEnemySkill(ToEnemySkill),
    #[default]
    None,
}

pub fn config_skill(c: &Config, os: &HashMap<String, OperatorRef>) {
    for (key, skill) in c.doctor["skill"].as_object().unwrap() {
        if let Some(value) = c.skill.get(key).unwrap().get(skill.as_str().unwrap()) {
            if let Some(o) = os.get(key) {
                o.borrow_mut()
                    .skill_ready
                    .push(serde_json::from_value(value.clone()).unwrap());
            } else {
                warn!("unknown operator name in skill config!")
            }
        } else {
            warn!("unknown skill name in skill config!,skill name:{}", skill)
        }
    }
}
impl Skill {
    pub fn ready(&self) -> bool {
        self.last != 0.0 && self.sp >= self.sp_cost
    }
    pub fn can_charge(&self) -> bool {
        self.sp < self.sp_cost || self.overcharge
    }
    pub fn charge(&mut self, value: f64) {
        if self.can_charge() {
            self.sp += value
        }
    }
}
