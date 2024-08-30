pub mod effect;
mod skill_fn;
pub mod skill_schedule;
pub mod skill_type;
use crate::unit::scope::Scope;
use crate::unit::skill::effect::Effect;
use serde::{Deserialize, Serialize};
use skill_type::*;

use super::enemy::EnemyShared;
use super::operator::OperatorShared;

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(default)]
pub(crate) struct Skill {
    pub(crate) trigger_type: TriggerType,
    pub(crate) schedule_type: ScheduleType,
    pub(crate) sp_data: SpData,
    ///skill time
    pub(crate) duration: f64,
    ///if in skill ,it show time remain,or is 0
    pub(crate) last: f64,
    pub(crate) skill_entity: SkillEntity,
}

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(default)]
pub(crate) struct SpData {
    pub(crate) sp_cost: f64,
    /// sp now
    pub(crate) sp: f64,
    pub(crate) overcharge: bool,
    pub(crate) charge_type: ChargeType,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
pub(crate) struct ToEnemySkill {
    #[serde(skip)]
    pub(crate) target: Vec<EnemyShared>,
    pub(crate) target_num: usize,
    pub(crate) effect: Effect,
    pub(crate) attack_type: AttackType,
    pub(crate) search_scope: Option<Scope>,
}

#[derive(Deserialize, Debug, Default, Clone, Serialize)]
pub(crate) struct ToOperatorSkill {
    #[serde(skip)]
    pub(crate) host: EnemyShared,
    #[serde(skip)]
    pub(crate) target: Vec<OperatorShared>,
    pub(crate) target_num: usize,
    pub(crate) effect: Effect,
    pub(crate) attack_type: AttackType,
    pub(crate) search_scope: Option<Scope>,
}

#[derive(Default, Deserialize, Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub(crate) enum SkillEntity {
    ToEnemySkill(ToEnemySkill),
    ToOperatorSkill(ToOperatorSkill),
    #[default]
    None,
}

// pub fn config_skill(c: &Config, os: &HashMap<String, OperatorRef>) {
//     for (key, skill) in c.doctor["skill"].as_object().unwrap() {
//         if let Some(value) = c.skill.get(key).unwrap().get(skill.as_str().unwrap()) {
//             if let Some(o) = os.get(key) {
//                 o.borrow_mut()
//                     .skill_ready
//                     .push(serde_json::from_value(value.clone()).unwrap());
//             } else {
//                 warn!("unknown operator name in skill config!")
//             }
//         } else {
//             warn!("unknown skill name in skill config!,skill name:{}", skill)
//         }
//     }
// }
