use std::cell::RefCell;
use crate::frame::{Frame, OperatorRef};
use crate::timeline::{Event,EventWithTime, read_doctor_timeline};
use crate::unit;
use crate::unit::operator::Operator;
use crate::utils::config::Config;
use crate::map;
use log::{info, trace, warn};
use serde_json::from_value;
use std::collections::{HashMap, VecDeque};
use std::fs::read;
use std::mem::forget;
use std::rc::Rc;
use serde::de::Unexpected::Map;
use crate::timeline::doctor::OperatorDeployEvent;
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
    event_set: Vec<Rc<dyn Event>>,
    pub route: Vec<Rc<Vec<Point>>>,
    time_remain: i64,
    pub enemy_initial: HashMap<String, Enemy>,
}

impl Calculator {
    pub fn next(&mut self) -> bool {
        if self.star != -1 {
            return false;
        }
        if self.time_remain == 0 {
            self.star = 0;
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
    pub fn process_frame(&mut self, f: &mut Frame) {
        self.event(f);
        f.step(self);
    }
    pub fn new(c: &Config) -> Result<Calculator> {
        use crate::timeline::hostile::EnemyPlaceEvent;
        use crate::unit::enemy::Enemy;
        use serde_json::from_value;
        let (mut time_line,event_set)=read_doctor_timeline(c)?;
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
            event_set,
            route,
            time_remain,
            enemy_initial,
        })
    }
    pub fn to_end(&mut self) {
        while self.next() {
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

}
