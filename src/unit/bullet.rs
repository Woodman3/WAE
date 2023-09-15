use std::cell::RefCell;
use std::rc::Rc;
use crate::unit::Unit;
use crate::utils::math::Point;

struct Bullet{
    target:Rc<RefCell<dyn Unit>>,
    direction:Point,
    location:Point,
}

impl Bullet {
    pub fn step(&mut self) {

    }
}