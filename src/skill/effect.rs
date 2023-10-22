use serde::Deserialize;
#[derive(Deserialize,Debug)]
pub(super) enum ChangeType{
    Absolute,
    Relative,
    None
}
#[derive(Deserialize,Debug)]
pub(super) enum ChangeClass{
    ASPD,
    ATK,
    DEF,
    MaxHP,
}
#[derive(Deserialize,Debug)]
pub(super) struct Effect{
    change_type:ChangeType,
    change_class:ChangeClass,
    value:f64,
}
