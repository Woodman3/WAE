use std::cell::RefCell;
use std::rc::Rc;
use env_logger::builder;
use crate::unit::Unit;
use crate::utils::math::{distance_from_segment_to_point, Point, to_target};

#[derive(Clone,Debug)]
pub struct Bullet{
    pub target:Rc<RefCell<dyn Unit>>,
    direction:Point,
    location:Point,
    move_speed:f64,
    pub distance:f64,
}

impl Bullet {
    pub fn step(&mut self,t:f64) {
        let target_point=self.target.borrow().get_loc();
        let (direction,new) = to_target(self.location,target_point,self.move_speed,t);
        self.distance = distance_from_segment_to_point(self.location, new, target_point);
    }
}
