use std::default;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Default, PartialEq, Serialize)]
pub enum ChargeType {
    Time,
    Attack,
    BeHit,
    #[default]
    None,
}
#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TriggerType {
    Auto,
    Manual,
    Passive,
    #[default]
    None,
}
#[derive(Clone, Copy, Deserialize, Debug, Default, Serialize)]
pub enum ScheduleType {
    PreEmptive,  //抢占式，会打断其他技能的释放，比如拔刀
    Immediately, //非抢占式，哪怕已经在释放其他技能也会立即释放，比如战术脉唱
    #[default]
    Delay, //非抢占式，会延迟到其他技能释放完毕后再释放，比如强力击
}

#[derive(Clone, Copy, Deserialize, Debug, Default, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttackType {
    #[default]
    None,
    All,
    Melee,
    Ranged,
}

pub(super) enum SkillStage {
    Standby,
    AnimationBefore(f64),
    AfterAttack(f64),
}

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
pub(crate) enum SkillExtra{
    #[default]
    DefaultAttack,
}