use std::cell::RefCell;
use crate::frame::{Frame, OperatorRef};
use crate::timeline::{Event, EventWithTime, read_timeline};
use crate::unit;
use crate::unit::operator::Operator;
use crate::utils::config::Config;
use crate::map;
use log::{trace, warn};
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use crate::unit::bullet::Bullet;
use crate::unit::enemy::Enemy;
use crate::utils::math::Point;

pub(crate) static PERIOD:f64=0.0166;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
/// calculate
#[derive(Debug)]
pub struct Calculator {
    pub frame_vec: Vec<Frame>,
    /// star in the battle we will get
    /// -1 mean battle haven't end
    star: i8,
    /// first element refer to time
    /// second element refer to event vector
    time_line: VecDeque<EventWithTime>,
    pub route: Vec<Rc<Vec<Point>>>,
    time_remain: i64,
    /// enemy in initial statement,if we place enemy to map,we will get enemy in it
    pub enemy_initial: HashMap<String, Enemy>,
    /// time that lase enemy may push,it isn't certainly because some enemy place by time
    last_enemy_time:u64
}

impl Calculator {
    pub fn step(&mut self) -> bool {
        if self.has_end(){
            self.star=-1;
            return false;
        }
        self.time_remain -= 1;
        if let Some(mut f) = self.frame_vec.pop() {
            f.timestamp += 1;
            self.process_frame(&mut f);
            self.frame_vec.push(f);
            true
        } else {
            false
        }
    }
    fn process_frame(&mut self, f: &mut Frame) {
        self.event(f);
        f.step(self);
    }
    pub fn new(c: &Config) -> Result<Calculator> {
        use serde_json::from_value;
        let (mut time_line,last_enemy_time)= read_timeline(c)?;
        time_line.make_contiguous().sort_by(|a,b|{
            a.time_stamp.cmp(&b.time_stamp)
        });
        let time_remain: i64 = from_value(c.hostile["time_remain"].clone())?;
        let mut route = Vec::<Rc<Vec<Point>>>::new();
        let temp: Vec<Vec<Vec<f64>>> = from_value(c.hostile["route"].clone())?;
        for v in temp {
            let mut r = Vec::<Point>::new();
            for c in v {
                r.push((c[0], c[1]).into());
            }
            route.push(Rc::new(r));
        }
        let mut frame_vec = Vec::<Frame>::new();
        let mut operator_undeploy = HashMap::<String, OperatorRef>::new();
        for (key, v) in c.operator.as_object().unwrap() {
            operator_undeploy.insert(key.clone(), Rc::new(RefCell::new(Operator::new(v)?)));
        }
        crate::unit::skill::config_skill(c, &operator_undeploy);
        frame_vec.push(Frame {
            timestamp: 0,
            enemy_set: Vec::<Rc::<RefCell::<Enemy>>>::new(),
            operator_deploy: HashMap::<String,OperatorRef>::new(),
            operator_undeploy,
            map:map::Map::new(&c.map)?,
            bullet_set:Vec::<Bullet>::new(),
            next_id:0,
        });
        let mut enemy_initial = HashMap::<String, unit::enemy::Enemy>::new();
        for (key, v) in c.enemy.as_object().unwrap() {
            enemy_initial.insert(key.clone(), Enemy::new(v)?);
        }
        Ok(Calculator {
            frame_vec,
            star: -1,
            time_line,
            route,
            time_remain,
            enemy_initial,
            last_enemy_time,
        })
    }
    pub fn goto_end(&mut self) {
        while self.step() {
            if let Some(f) = self.frame_vec.last() {
                if f.timestamp%10==0{
                    trace!("{}", f);
                }
            }
        }

    }
    /// an event is place event or enemy comeout or something happen like fire rain
    /// mosttime is happen in an specify time but sometime it happen after somethine has happen
    /// it can't be skip
    fn event(&mut self, f: &mut Frame) {
        while self.time_line.len() != 0 {
            if let Some(et) = self.time_line.front() {
                if et.time_stamp != f.timestamp {
                    if et.time_stamp < f.timestamp {
                        warn!("Some event not happened before,this event has drop");
                        self.time_line.pop_front();
                        continue;
                    } else {
                        break;
                    }
                } else {
                    et.e.happen(f, self);
                    self.time_line.pop_front();
                }
            }
        }
    }
    pub(super) fn has_end(&self)->bool{
        self.star!=-1||self.time_remain==0
    }

    pub(super) fn get_obs(&self)->Option<Value>{
        if let Some(f) = self.frame_vec.last(){
           Some(f.get_obs())
        }else{
            None
        }
    }

    pub(super) fn get_acs(&self)->Option<Value>{
        if let Some(f) = self.frame_vec.last(){
           Some(f.get_acs())
        }else{
            None
        }
    }
    
    pub(super) fn get_frame(&self)->Option<&Frame>{
        self.frame_vec.last()
    }

    pub(super) fn insert_event(&mut self,e:Rc<dyn Event>)->bool{
        if let Some(f) = self.frame_vec.last(){
            let time = f.timestamp+1;
            let et =EventWithTime{time_stamp:time,e:e};
            self.time_line.push_front(et);
            return true
        }         
        false
    }
}

#[cfg(test)]
mod test{
}