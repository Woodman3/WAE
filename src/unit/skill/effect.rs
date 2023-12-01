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
#[derive(Clone,Deserialize,Debug)]
pub struct Buff{
    pub change_type:ChangeType,
    pub change_class:ChangeClass,
    pub value:f64,
}
#[derive(Clone,Deserialize,Debug,Default)]
#[serde(untagged)]
pub enum Effect{
    Buff(Buff),
    Damage(Damage),
    #[default]
    None,
}

// enum ReferType{
//     MySelf,
//     Friend,
//     Friends,
//     Enemy,
//     Enemies,
// }


#[derive(Debug,Clone,Deserialize)]
pub struct Damage{
    pub value:f64,
    pub damage_type:DamageType,
}

#[derive(Debug,Clone,Copy,Deserialize,Default)]
pub enum DamageType {
    #[default]
    Physical,
    Magic,
    Real,
}
