use serde::Deserialize;
#[derive(Clone,Deserialize,Debug)]
pub(super) enum ChangeType{
    Absolute,
    Relative,
    None
}
#[derive(Clone,Deserialize,Debug)]
pub(super) enum ChangeClass{
    ASPD,
    ATK,
    DEF,
    MaxHP,
}
#[derive(Clone,Deserialize,Debug)]
pub struct Effect{
    change_type:ChangeType,
    change_class:ChangeClass,
    value:f64,
}
