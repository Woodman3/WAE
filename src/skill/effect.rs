use serde::Deserialize;
#[derive(Deserialize,Debug)]
pub(super) enum ChangeType{
    Absolute,
    Relative,
}
#[derive(Deserialize,Debug)]
pub(super) enum ChangeClass{
    AttackSpeed,
    AttackPower,
}
#[derive(Deserialize,Debug)]
pub(super) struct Effect{
    change_type:ChangeType,
    change_class:ChangeClass,
    value:f64,
}
