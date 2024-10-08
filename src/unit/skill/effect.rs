use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub(crate) enum ChangeType {
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
pub(crate) enum ChangeClass {
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
pub(crate) struct Buff {
    pub(crate) change_type: ChangeType,
    pub(crate) change_class: ChangeClass,
    pub(crate) value: f64,
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

// i don't know why i make this struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct FixedDamage {
    pub(crate) value: i64,
    pub(crate) damage_type: DamageType,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub(crate) struct Damage {
    // #[serde(skip)] //from operator ,don't need to set
    pub(crate) value: i64,
    pub(crate) change: Option<Change>,
    pub(crate) damage_type: DamageType,
}
#[derive(Debug, Clone, Copy, Deserialize, Default, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum DamageType {
    #[default]
    None,
    Physical,
    Magical,
    Heal,
    Real,
}
