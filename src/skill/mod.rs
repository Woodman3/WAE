mod effect;
use effect::Effect;
use serde::Deserialize;
#[derive(Deserialize,Debug)]
pub enum RestoreType {
    Auto,
    Attack,
    Hit
}
#[derive(Deserialize,Debug)]
pub enum TriggerType{
    Auto,
    Manual
}
#[derive(Deserialize,Debug)]
pub struct Skill{
    restore_type:RestoreType,
    trigger_type:TriggerType,
    effect:Vec<Effect>
}
