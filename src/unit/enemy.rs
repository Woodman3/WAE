use crate::utils::math;
use serde_json::Value;
use std::fmt;
use std::rc::Rc;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug, Clone)]
pub struct Enemy {
    info: super::UnitInfo,
    pub location: (f64, f64),
    /// -1 mean haven't place
    pub target: (f64, f64),
    move_speed: f64,
    component_x: f64,
    ///compoment of vector
    component_y: f64,
    route_stage: usize,
    pub die_code: u32,
    /// 0 mean haven't die
    // route:Option<&'a Vec<Vec<(f64,f64)>>>
    pub route: Option<Rc<Vec<(f64, f64)>>>,
}

impl Enemy {
    /// t is 1/fps it mean time interval
    pub fn step(&mut self, t: f64) {
        let mut new = self.location.clone();
        new.0 += self.move_speed as f64 * self.component_x * t;
        new.1 += self.move_speed as f64 * self.component_y * t;
        let distance = math::distance_from_segment_to_point(&self.location, &new, &self.target);
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
            location: (-1f64, -1f64),
            target: (-1f64, -1f64),
            move_speed: serde_json::from_value::<f64>(v["move_speed"].clone())?,
            route_stage: 1,
            component_x: 0f64,
            component_y: 0f64,
            die_code: 0,
            route: None,
        })
    }
    pub fn calculate_vector(&mut self) {
        let (delta_x, delta_y) = crate::sub2d!(self.target, self.location);
        let theta = delta_y.atan2(delta_x);
        self.component_x = theta.cos();
        self.component_y = theta.sin();
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
            self.location.0,
            self.location.1,
            self.component_x,
            self.component_y,
            self.target.0,
            self.target.1
        )
    }
}
