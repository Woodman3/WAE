use std::cell::RefCell;
use std::cmp::Ordering;
use crate::utils::math;
use serde_json::Value;
use std::fmt;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use log::{error, trace, warn};
use crate::calculator::PERIOD;
use crate::frame::{Frame, OperatorRef};
use crate::unit::bullet::Bullet;
use crate::unit::{Unit, UnitInfo};
use crate::unit::code::DIE;
use crate::unit::damage::Damage;
use crate::unit::operator::Operator;
use crate::utils::math::{Grid, Point, to_target};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug, Clone)]
pub struct Enemy {
    pub name:String,
    info: super::UnitInfo,
    pub stage:super::UnitInfo,
    pub location: Point,
    /// -1 mean haven't place
    pub next_point: Point,
    move_speed: f64,
    direction:Point,
    route_stage: usize,
    pub die_code: u32,
    /// 0 mean haven't die
    pub route: Option<Rc<Vec<Point>>>,
    pub be_block:Weak<RefCell<Operator>>,
    pub identifier:usize,
}
#[derive(Debug,Clone)]
pub struct EnemyWithPriority{
    pub enemy:Weak<RefCell<Enemy>>,
    pub time_stamp:u64,
}
impl Enemy {
    /// t is 1/fps it mean time interval
    pub fn step(&mut self) {
        let (direction,new) = to_target(self.location, self.next_point, self.move_speed);
        let distance = math::distance_from_segment_to_point(self.location, new, self.next_point);
        if distance <= super::code::MIN_DISTANCE {
            self.route_stage += 1;
            if let Some(route) = &self.route {
                if let Some(target) = route.get(self.route_stage) {
                    self.next_point = target.clone();
                } else {
                    self.die_code = super::code::INTO_END;
                }
            }
        }
        self.direction=direction;
        self.location = new;
    }
    pub fn attack(&mut self,bv:&mut Vec<Bullet>,o:OperatorRef){
        if self.stage.attack_time>0.0{
            self.stage.attack_time-=PERIOD;
        }else {
            match self.stage.attack_type.as_str() {
                "Melee"=>{
                    let d=Damage{
                        value:self.stage.damage,
                        damage_type:self.stage.damage_type.clone(),
                    };
                    o.borrow_mut().be_damage(&d);
                    // self.target.upgrade().unwrap().borrow_mut().be_damage(&d);
                }
                "Ranged"=>{
                    //todo: ranged enemy
                    // bv.push(Bullet::new(
                    //     self.target.upgrade().unwrap(),
                    //     Point::from(self.location),
                    //     2f64,
                    //     self.stage.damage_type.clone(),
                    //     self.stage.damage,
                    // ));
                }
                _ => {error!("unknown attack_type!")}
            }
            self.stage.attack_time=self.info.attack_time;
        }
    }
    pub fn next(&mut self,f:&mut Frame){
        if let Some(o)=self.be_block.upgrade(){
            self.attack(&mut f.bullet_set,o);
        }else{
            self.step();
        }
        if self.stage.health<=0.0{
            self.die_code=DIE;
        }
    }
    pub fn new(v: &Value) -> Result<Enemy> {
        let info = serde_json::from_value::<super::UnitInfo>(v["UnitInfo"].clone())?;
        let stage=info.clone();
        Ok(Enemy {
            name:serde_json::from_value(v["name"].clone())?,
            info,
            stage,
            location: (-1f64, -1f64).into(),
            next_point: (-1f64, -1f64).into(),
            move_speed: serde_json::from_value::<f64>(v["move_speed"].clone())?,
            route_stage: 1,
            direction: (0.0, 0.0).into(),
            die_code: 0,
            route: None,
            be_block: Weak::new(),
            identifier: 0,
        })
    }
}

impl fmt::Display for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
            component_x:{} component_y:{}\n\
            health:{} \n\
            ",
            self.direction.x,
            self.direction.y,
            self.stage.health,
        )
    }
}


impl Unit for Enemy{
    fn get_loc(&self) -> Point {
        self.location
    }
    fn be_hit(&mut self, b: &Bullet, f: &mut Frame) {
        self.be_damage(&b.damage);
    }
    fn be_damage(&mut self, d: &Damage) {
        match d.damage_type.as_str() {
            "Magic" =>{
                let damage=d.value*(1f64-self.stage.magic_resist);
                self.stage.health-=damage;
            }
            "Physical"=>{
                let damage=d.value-self.stage.armor;
                self.stage.health-=damage;
            }
            "Real"=>{
                self.stage.health-=d.value;
            }
            _ => {
                warn!("unknown attack type of bullet ,bullet has been departure");
                return
            }
            &_ => {}
        }
        if self.stage.health<=0.0{
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