use std::default;

use serde::{Deserialize, Serialize};

use super::utils::math::Point;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(super) struct Route {
    pub(super) start: Point,
    pub(super) end: Point,
    pub(super) checkpoints: Vec<CheckPoint>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone,PartialEq)]
pub(super) enum CheckPoint {
    Move(Point),
    // WaitForSeconds(f64),
    // Disappear,
    // AppearAtPos(Point),
    // WaitCurrentFragmentTime(f64),
    // WaitCurrentWaveTime(f64),
    // PatrolMove(Point),
    #[serde(other)]
    #[default]
    None,
}
