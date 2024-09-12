use std::cell::RefCell;

use crate::event::Event;
use crate::frame::{Frame, OperatorRef};
use crate::map;
use crate::route::Route;
use crate::spawner::Spawner;
use crate::unit;
use crate::unit::enemy::Enemy;
use crate::unit::operator::Operator;
use crate::utils::config::Config;
use crate::utils::copilot::Copilot;
use log::debug;
use serde_json::Value;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) static PERIOD: f64 = 0.0166;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
/// calculate
#[derive(Debug, Default)]
pub struct Calculator {
    pub(super) frame_vec: Vec<Frame>,
    /// star in the battle we will get
    /// -1 mean battle haven't end
    pub(super) star: i8,
    pub(super) event_buffer: Vec<Event>,
    pub(super) route: Vec<Route>,
    pub(super) time_remain: i64,
    /// enemy in initial statement,if we place enemy to map,we will get enemy in it
    pub(super) enemy_initial: HashMap<String, Enemy>,
    pub(super) copilot: Option<Copilot>,
    pub(super) spawner: Spawner,
}

impl Calculator {
    pub(super) fn step(&mut self) -> bool {
        if self.has_end() {
            self.star = -1;
            return false;
        }
        self.time_remain -= 1;
        if let Some(mut f) = self.frame_vec.pop() {
            f.timer.step();
            self.process_frame(&mut f);
            self.frame_vec.push(f);
            true
        } else {
            false
        }
    }
    fn process_frame(&mut self, f: &mut Frame) {
        let mut ev = std::mem::take(&mut self.event_buffer);
        if let Some(copilot) = &self.copilot {
            ev.extend(copilot.query(f));
        }
        ev.extend(self.spawner.step(f));
        f.events.extend(ev);
        // self.event_buffer.extend(ev);
        // self.event(f);
        f.step(self);
    }
    pub(super) fn new(c: &Config) -> Result<Calculator> {
        use serde_json::from_value;
        let time_remain: i64 = from_value(c.hostile["time_remain"].clone())?;
        let route = Vec::<Route>::new();
        let mut frame_vec = Vec::<Frame>::new();
        let mut operator_undeploy = HashMap::<String, OperatorRef>::new();
        for (key, v) in c.operator.as_object().unwrap() {
            operator_undeploy.insert(key.clone(), Rc::new(RefCell::new(Operator::new(v)?)));
        }
        // crate::unit::skill::config_skill(c, &operator_undeploy);
        frame_vec.push(Frame {
            operator_undeploy,
            map: map::Map::new(&c.map)?,
            next_id: 0,
            ..Default::default()
        });
        let mut enemy_initial = HashMap::<String, unit::enemy::Enemy>::new();
        for (key, v) in c.enemy.as_object().unwrap() {
            enemy_initial.insert(key.clone(), Enemy::new(v)?);
        }
        Ok(Calculator {
            frame_vec,
            star: -1,
            route,
            time_remain,
            enemy_initial,
            ..Default::default()
        })
    }
    pub(super) fn goto_end(&mut self) {
        while self.step() {
            if let Some(f) = self.frame_vec.last() {
                debug!("{}", f);
            }
        }
    }
    /// an event is place event or enemy appear or something happen like fire rain
    /// mostly is happen in an specify time but sometime it happen after something has happen
    /// it can't be skip
    fn event(&mut self, f: &mut Frame) {
        let ev = std::mem::take(&mut self.event_buffer);
        for e in ev {
            e.happen(f, self);
        }
    }
    pub(super) fn has_end(&self) -> bool {
        self.star != -1 || self.time_remain == 0 
    }

    pub(super) fn get_obs(&self) -> Option<Value> {
        if let Some(f) = self.frame_vec.last() {
            Some(f.get_obs())
        } else {
            None
        }
    }

    pub(super) fn get_acs(&self) -> Option<Value> {
        if let Some(f) = self.frame_vec.last() {
            Some(f.get_acs())
        } else {
            None
        }
    }

    pub(super) fn get_frame(&self) -> Option<&Frame> {
        self.frame_vec.last()
    }
}

#[cfg(test)]
mod test {}
