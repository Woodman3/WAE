use crate::unit::Unit;
use crate::utils::math::{distance_from_segment_to_point, to_target, Point};
use env_logger::builder;
use std::cell::RefCell;
use std::f64::MAX;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::unit::damage::Damage;

#[derive(Clone, Debug)]
pub struct Bullet {
    pub target: Rc<RefCell<dyn Unit>>,
    direction: Point,
    pub location: Point,
    move_speed: f64,
    pub distance: f64,
    pub damage:Damage,
}

impl Bullet {
    pub fn step(&mut self) {
        let target_point = self.target.borrow().get_loc();
        let (direction, new) = to_target(self.location, target_point, self.move_speed);
        self.distance = distance_from_segment_to_point(self.location, new, target_point);
        self.location=new;
    }
    pub fn new(target: Rc<RefCell<dyn Unit>>, location: Point, move_speed: f64,
    damage_type:String,damage:f64) -> Self {
        Bullet {
            target,
            direction: (0.0, 0.0).into(),
            location,
            move_speed,
            distance: f64::MAX,
            damage:Damage{
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
