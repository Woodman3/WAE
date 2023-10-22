mod effect;
use effect::Effect;
use serde::Deserialize;
#[derive(Deserialize,Debug)]
pub enum ChargeType {
    Auto,
    Attack,
    Hit,
    Passive,
}
#[derive(Deserialize,Debug)]
pub enum TriggerType{
    Auto,
    Manual,
    Passive,
}
#[derive(Deserialize,Debug)]
pub struct Skill{
    charge_type: ChargeType,
    trigger_type:TriggerType,
    duration:f64,
    sp_cost:f64,
    sp_initial:f64,
    effect:Vec<Effect>
}
