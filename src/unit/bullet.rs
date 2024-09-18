use serde::{Deserialize, Serialize};

use crate::unit::Unit;
use crate::utils::math::{distance_from_segment_to_point, to_target, Point};

use std::fmt::{Display, Formatter};

use super::skill::effect::Effect;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Bullet {
    pub(crate) target: Unit,
    pub(crate) direction: Point,
    pub(crate) location: Point,
    pub(crate) move_speed: f64,
    pub(crate) distance: f64,
    pub(crate) effect: Effect,
}

impl Bullet {
    pub(crate) fn step(&mut self) {
        let target_point = self.target.get_loc();
        let (_direction, new) = to_target(self.location, target_point, self.move_speed);
        self.distance = distance_from_segment_to_point(self.location, new, target_point);
        self.location = new;
    }
}

impl Display for Bullet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\
        distance to target:{}\n",
            self.distance
        )
    }
}
