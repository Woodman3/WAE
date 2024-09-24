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
    pub(crate) duration: f32,
    ///if in skill ,it show time remain,or is 0
    pub(crate) last: f32,
    pub(crate) skill_entity: SkillEntity,
    pub(crate) extra:Option<SkillExtra>
}

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(default)]
pub(crate) struct SpData {
    pub(crate) sp_cost: f32,
    /// sp now
    pub(crate) sp: f32,
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
