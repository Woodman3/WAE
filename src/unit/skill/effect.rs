use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum ChangeType {
    ///directly add
    DA,
    ///directly mul
    DM,
    ///lastly add
    LA,
    ///lastly mul
    LM,
}
#[derive(Clone, Deserialize, Debug, Serialize)]
pub enum ChangeClass {
    ASPD,
    ATK,
    DEF,
    MaxHP,
    Hp,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub(crate) struct Change {
    pub(super) change_type: ChangeType,
    pub(super) change_class: ChangeClass,
}

#[derive(Default, Deserialize, Debug, Clone, Serialize)]
pub(crate) enum TargetType {
    Operator,
    Enemy,
    #[default]
    MySelf,
    Friend,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Buff {
    pub change_type: ChangeType,
    pub change_class: ChangeClass,
    pub value: f64,
}
#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(tag = "type")]
pub(crate) enum Effect {
    Buff(Buff),
    FixedDamage(FixedDamage),
    Damage(Damage),
    #[default]
    None,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FixedDamage {
    pub value: i64,
    pub damage_type: DamageType,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub(crate) struct Damage {
    #[serde(skip)] //from operator ,don't need to set
    pub(super) value: i64,
    pub(super) change: Option<Change>,
}
#[derive(Debug, Clone, Copy, Deserialize, Default, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DamageType {
    #[default]
    None,
    Physical,
    Magical,
    Heal,
    Real,
}
