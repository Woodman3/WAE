use crate::frame::Frame;
use crate::timeline::Event;
use crate::unit;
use crate::unit::operator::Operator;
use crate::utils::config::Config;
use log::{trace, warn};
use serde_json::from_value;
use std::collections::{HashMap, VecDeque};
use std::mem::forget;
use std::rc::Rc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
/// calculate
#[derive(Debug)]
pub struct Calculator {
    frame_vec: Vec<Frame>,
    /// star in the battle we will get
    /// -1 mean battle haven't end
    star: i8,
    /// first element refer to time
    /// second element refer to event vector
    time_line: VecDeque<(u64,String)>,
    event_set: HashMap<String,Box<dyn Event>>,
    pub route: Vec<Rc<Vec<(f64, f64)>>>,
    time_remain: i64,
    pub enemy_initial: HashMap<String, unit::enemy::Enemy>,
}

impl Calculator {
    fn next(&mut self) -> bool {
        if self.star != -1 {
            return false;
        }
        if self.time_remain == 0 {
            self.star = 0;
            return false;
        }
        self.time_remain -= 1;
        if let Some(f) = self.frame_vec.last() {
            let mut f = f.clone();
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
        f.step(self, 0.01);
    }
    pub fn new(c: &Config) -> Result<Calculator> {
        use crate::timeline::hostile::EnemyPlaceEvent;
        use crate::unit::enemy::Enemy;
        use serde_json::from_value;
        let mut event_set = HashMap::<String,Box<dyn Event>>::new();
        let mut time_line = VecDeque::<(u64, String)>::new();
        let time_remain: i64 = from_value(c.hostile["time_remain"].clone())?;
        let mut route = Vec::<Rc<Vec<(f64, f64)>>>::new();
        // let temp: Vec<Vec<u64>> = from_value(c.hostile["timeline"].clone())?;
        for v in c.hostile["timeline"].as_array().unwrap() {
            time_line.push_back((from_value::<u64>(v[0].clone())?, from_value::<String>(v[1].clone())?));
        }
        let temp: Vec<Vec<Vec<f64>>> = from_value(c.hostile["route"].clone())?;
        for v in temp {
            let mut r = Vec::<(f64, f64)>::new();
            for c in v {
                r.push((c[0], c[1]));
            }
            route.push(Rc::new(r));
        }
        for (k,v) in c.hostile["event"].as_object().unwrap() {
            let e: EnemyPlaceEvent = from_value(v.clone())?;
            event_set.insert(k.clone(),Box::new(e));
        }
        let mut frame_vec = Vec::<Frame>::new();
        let mut operator_undeploy = HashMap::<String, Operator>::new();
        for (key, v) in c.operator.as_object().unwrap() {
            operator_undeploy.insert(key.clone(), Operator::new(v)?);
        }
        frame_vec.push(Frame {
            timestamp: 0,
            enemy_set: Vec::<Enemy>::new(),
            operator_deploy: HashMap::<String,Operator>::new(),
            operator_undeploy,
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
                // println!("{}",f);
                trace!("{}", f);
            }
        }

    }
    /// an event is place event or enemy comeout or something happen like fire rain
    /// mosttime is happen in an specify time but sometime it happen after somethine has happen
    /// it can't be skip
    fn event(&mut self, f: &mut Frame) {
        let time_stamp = f.timestamp;
        while self.time_line.len() != 0 {
            if let Some((time, e)) = self.time_line.front() {
                if *time != time_stamp {
                    if *time < time_stamp {
                        warn!("Some event not happened before,this event has drop");
                        self.time_line.pop_front();
                        continue;
                    } else {
                        break;
                    }
                } else {
                    self.event_set[e].happen(f, self);
                    self.time_line.pop_front();
                }
            }
        }
    }

    // fn enemy_move(&self,f:mut &Frame){
    // }
}
