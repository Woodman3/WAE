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
    PreEmptive,
    Immediately,
    #[default]
    Delay
}

#[derive(Clone,Copy,Deserialize,Debug,Default)]
pub enum AttackType{
    #[default]
    Melee,
    Ranged
}