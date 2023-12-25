use std::cmp::Ordering;
use serde::Deserialize;
#[derive(Clone,Deserialize,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub enum ChangeType{
    ///directly add
    DA,
    ///directly mul
    DM,
    ///lastly add
    LA,
    ///lastly mul
    LM,
}
#[derive(Clone,Deserialize,Debug)]
pub enum ChangeClass{
    ASPD,
    ATK,
    DEF,
    MaxHP,
    Hp,
}

#[derive(Deserialize,Debug,Clone)]
pub(crate) struct Change{
    pub(super) change_type:ChangeType,
    pub(super) change_class:ChangeClass,
}

#[derive(Default,Deserialize,Debug,Clone)]
pub(crate) enum TargetType{
    Operator,
    Enemy,
    #[default]
    MySelf,
    Friend,
}

#[derive(Clone,Deserialize,Debug)]
pub struct Buff{
    pub change_type:ChangeType,
    pub change_class:ChangeClass,
    pub value:f64,
}
#[derive(Clone,Deserialize,Debug,Default)]
#[serde(tag="type")]
pub(crate) enum Effect{
    Buff(Buff),
    FixedDamage(FixedDamage),
    Damage(Damage),
    #[default]
    None,
}

#[derive(Debug,Clone,Deserialize)]
pub struct FixedDamage {
    pub value:f64,
    pub damage_type:DamageType,
}

#[derive(Clone,Deserialize,Debug)]
pub(crate) struct Damage{
    #[serde(skip)]//from operator ,don't need to set
    pub(super) value:f64,
    pub(super) change:Option<Change>,
}
#[derive(Debug,Clone,Copy,Deserialize,Default)]
pub enum DamageType {
    #[default]
    Physical,
    Magic,
    Real,
}
