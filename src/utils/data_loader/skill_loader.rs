use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Copy)]
#[serde(rename_all = "snake_case")]
pub(super) enum SkillKey {
    Atk,
    /// 攻击力倍率
    AtkScale,
    AttackSpeed,
    /// 连续发动次数
    Times,
    #[default]
    #[serde(other)]
    None,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialBlackBoard {
    pub(super) key: SkillKey,
    pub(super) value: f32,
    pub(super) value_str: Option<String>,
}
