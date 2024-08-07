use std::default;

use serde::{Deserialize, Serialize};

use super::utils::math::Point;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct Route {
    start: Point,
    end: Point,
    checkpoints: Vec<CheckPoint>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
enum CheckPoint {
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
