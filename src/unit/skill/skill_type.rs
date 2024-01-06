use serde::Deserialize;

#[derive(Clone,Deserialize,Debug,Default,PartialEq)]
pub enum ChargeType {
    Auto,
    Attack,
    BeHit,
    Passive,
    #[default]
    None
}
#[derive(Clone,Deserialize,Debug,Default)]
pub enum TriggerType{
    Auto,
    Manual,
    Passive,
    #[default]
    None
}
#[derive(Clone,Copy,Deserialize,Debug,Default)]
pub enum ScheduleType{
    PreEmptive,//抢占式，会打断其他技能的释放，比如拔刀
    Immediately,//非抢占式，哪怕已经在释放其他技能也会立即释放，比如战术脉唱
    #[default]
    Delay//非抢占式，会延迟到其他技能释放完毕后再释放，比如强力击
}

#[derive(Clone,Copy,Deserialize,Debug,Default)]
pub enum AttackType{
    #[default]
    Melee,
    Ranged
}

pub(super) enum SkillStage{
    Standby,
    AnimationBefore(f64),
    AfterAttack(f64),
}