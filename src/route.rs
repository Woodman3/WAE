
use serde::{Deserialize, Serialize};

use crate::{map::Map, utils::math::Grid};

use super::utils::math::Point;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub(super) struct Route {
    pub(super) start: Point,
    pub(super) end: Point,
    pub(super) checkpoints: Vec<CheckPoint>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
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

impl Route {
    pub(super) fn complete(&mut self, m: &Map) {
        let mut start: Grid = self.start.into();
        let mut ex_checkpoints = Vec::new();
        for c in self.checkpoints.iter_mut() {
            if let CheckPoint::Move(p) = c {
                let end = p.clone().into();
                let path = m.spfa(start, end);
                for p in path {
                    ex_checkpoints.push(CheckPoint::Move(p.into()));
                }
                start = end;
            } else {
                ex_checkpoints.push(c.clone());
            }
        }
    }
}
