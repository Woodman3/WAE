use std::cell::RefCell;
use std::cmp::Ordering;
use crate::utils::math;
use serde_json::Value;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use log::{trace, warn};
use crate::calculator::ENEMY_IDENTIFIER;
use crate::frame::Frame;
use crate::unit::bullet::Bullet;
use crate::unit::{Unit, UnitInfo};
use crate::utils::math::{Grid, Point, to_target};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug, Clone)]
pub struct Enemy {
    pub name:String,
    info: super::UnitInfo,
    pub stage:super::UnitInfo,
    pub location: Point,
    /// -1 mean haven't place
    pub target: Point,
    move_speed: f64,
    direction:Point,
    route_stage: usize,
    pub die_code: u32,
    /// 0 mean haven't die
    pub route: Option<Rc<Vec<Point>>>,
    be_block:Option<String>,
    pub identifier:u64,
}
#[derive(Debug,Clone)]
pub struct EnemyWithPriority{
    pub enemy:Rc<RefCell<Enemy>>,
    pub time_stamp:u64,
}
impl Enemy {
    /// t is 1/fps it mean time interval
    pub fn step(&mut self) {
        // let mut new = self.location.clone();
        // new.x += self.move_speed * self.direction.x * t;
        // new.y += self.move_speed * self.direction.y * t;
        let (direction,new) = to_target(self.location,self.target,self.move_speed);
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
        self.direction=direction;
        self.location = new;
    }
    pub fn new(v: &Value) -> Result<Enemy> {
        let info = serde_json::from_value::<super::UnitInfo>(v["UnitInfo"].clone())?;
        let stage=info.clone();
        unsafe {
            ENEMY_IDENTIFIER += 1;
            Ok(Enemy {
                name:serde_json::from_value(v["name"].clone())?,
                info,
                stage,
                location: (-1f64, -1f64).into(),
                target: (-1f64, -1f64).into(),
                move_speed: serde_json::from_value::<f64>(v["move_speed"].clone())?,
                route_stage: 1,
                direction: (0.0, 0.0).into(),
                die_code: 0,
                route: None,
                be_block: None,
                identifier: ENEMY_IDENTIFIER,
            })
        }
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
    fn be_hit(&mut self, b: &Bullet, f: &mut Frame) {
        match b.attack_type.as_str() {
            "Magic" =>{
                let damage=b.damage*(1f64-self.stage.magic_resist);
                self.stage.health-=damage;
            }
            "Physical"=>{
                let damage=b.damage-self.stage.armor;
                self.stage.health-=damage;
            }
            "Real"=>{
                self.stage.health-=b.damage;
            }
            _ => {
                warn!("unknown attack type of bullet ,bullet has been departure");
                return
            }
        }
        if self.stage.health<=0f64{
            self.die_code=super::code::DIE;
            trace!("an enemy has die!");
            return;
        }
    }
         
}

impl PartialEq<Self> for Enemy {
    fn eq(&self, other: &Self) -> bool {
        self.identifier==other.identifier
    }
}

impl Eq for Enemy{}

impl Eq for EnemyWithPriority {}

impl PartialEq<Self> for EnemyWithPriority {
    fn eq(&self, other: &Self) -> bool {
        self.time_stamp==other.time_stamp
    }
}

impl PartialOrd<Self> for EnemyWithPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.time_stamp.partial_cmp(&other.time_stamp)
    }
}

impl Ord for EnemyWithPriority{
    fn cmp(&self, other: &Self) -> Ordering {
        self.time_stamp.cmp(&other.time_stamp)
    }
}