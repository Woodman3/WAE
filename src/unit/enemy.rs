use std::cell::RefCell;
use crate::utils::math;
use serde_json::Value;
use std::fmt;
use std::rc::Rc;
use crate::unit::Unit;
use crate::utils::math::Point;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug, Clone)]
pub struct Enemy {
    info: super::UnitInfo,
    pub location: Point,
    /// -1 mean haven't place
    pub target: Point,
    move_speed: f64,
    direction:Point,
    route_stage: usize,
    pub die_code: u32,
    /// 0 mean haven't die
    pub route: Option<Rc<Vec<Point>>>,
}
#[derive(Debug)]
pub struct EnemyWithPriority{
    pub enemy:Rc<RefCell<Enemy>>,
    pub time_stamp:u64,
}

impl Enemy {
    /// t is 1/fps it mean time interval
    pub fn step(&mut self, t: f64) {
        let mut new = self.location.clone();
        new.x += self.move_speed * self.direction.x * t;
        new.y += self.move_speed * self.direction.y * t;
        let distance = math::distance_from_segment_to_point(self.location, new, self.target);
        if (distance <= super::code::MIN_DISTANCE) {
            self.route_stage += 1;
            if let Some(route) = &self.route {
                if let Some(target) = route.get(self.route_stage) {
                    self.target = target.clone();
                } else {
                    self.die_code = super::code::INTO_END;
                }
            }
        }
        self.location = new;
    }
    pub fn new(v: &Value) -> Result<Enemy> {
        Ok(Enemy {
            info: serde_json::from_value::<super::UnitInfo>(v["UnitInfo"].clone())?,
            location: (-1f64, -1f64).into(),
            target: (-1f64, -1f64).into(),
            move_speed: serde_json::from_value::<f64>(v["move_speed"].clone())?,
            route_stage: 1,
            direction:(0.0,0.0).into(),
            die_code: 0,
            route: None,
        })
    }
    pub fn calculate_vector(&mut self) {
        let delta = self.target-self.location;
        let theta = delta.y.atan2(delta.x);
        self.direction=(theta.cos(),theta.sin()).into();
    }
}

impl fmt::Display for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
location:{},{}
component_x:{} component_y:{}
target:{},{}",
            self.location.x,
            self.location.y,
            self.direction.x,
            self.direction.y,
            self.target.x,
            self.target.y
        )
    }
}

impl Unit for Enemy{
    fn get_loc(&self) -> Point {
        self.location
    }
}
