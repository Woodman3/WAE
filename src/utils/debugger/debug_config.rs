use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub(super) struct DebugConfig{
    pub(super) operator: bool,
    pub(super) enemy: bool,
    pub(super) timer:(bool,TimerConfig)
}

#[derive(Deserialize, Debug, Default)]
pub(super) struct TimerConfig{
    pub(super) global: bool,
    pub(super) subwave: bool,
    pub(super) wave: bool,
}

#[derive(Deserialize, Debug, Default)]
pub(super) struct OperationConfig{
    pub(super) skills: bool,
}

