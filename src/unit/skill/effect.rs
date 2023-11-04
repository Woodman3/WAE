use serde::Deserialize;
#[derive(Clone,Deserialize,Debug)]
pub enum ChangeType{
    Absolute,
    Relative,
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

enum Effect{
    Buff(Buff),
    Damage(Damage),
}

enum ReferType{
    MySelf,
    Friend,
    Friends,
    Enemy,
    Enemies,
}


#[derive(Debug,Clone)]
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
