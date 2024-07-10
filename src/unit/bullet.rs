use serde::{Deserialize, Serialize};

use crate::unit::Unit;
use crate::utils::math::{distance_from_segment_to_point, to_target, Point};

use std::fmt::{Display, Formatter};

use crate::unit::skill::effect::{FixedDamage, DamageType};

#[derive(Clone, Debug,Serialize,Deserialize)]
pub(super)  struct Bullet {
    pub(super) target: Unit,
    direction: Point,
    pub(super) location: Point,
    move_speed: f64,
    pub(super) distance: f64,
    pub(super) damage: FixedDamage,
}

impl Bullet {
    pub(super) fn step(&mut self) {
        let target_point = self.target.get_loc();
        let (_direction, new) = to_target(self.location, target_point, self.move_speed);
        self.distance = distance_from_segment_to_point(self.location, new, target_point);
        self.location=new;
    }

    pub(super) fn new(target:Unit, location: Point, move_speed: f64,
    damage_type:DamageType,damage:i64) -> Self {
        Bullet {
            target,
            direction: (0.0, 0.0).into(),
            location,
            move_speed,
            distance: f64::MAX,
            damage: FixedDamage {
                value:damage,
                damage_type,
            }
        }
    }
}

impl Display for Bullet{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"\
        distance to target:{}\n",
        self.distance)
    }
}
