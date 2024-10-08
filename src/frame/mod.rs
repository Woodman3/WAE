pub(crate) mod timer;

use crate::calculator::{Calculator, PERIOD};
use crate::event::Event;
use crate::map::{self, Map};
use crate::unit::bullet::Bullet;
use crate::unit::enemy::Enemy;
use crate::unit::operator::Operator;
use crate::unit::skill::skill_schedule::SkillSchedule;
use crate::unit::{code, skill};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use timer::Timer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, write};
use std::rc::Rc;

pub(super) type OperatorRef = Rc<RefCell<Operator>>;
pub(super) type EnemyRef = Rc<RefCell<Enemy>>;

#[derive(Debug, Default, Clone, Serialize)]
pub(super) struct Frame {
    pub(super) timer: Timer,
    pub(super) enemy_set: Vec<EnemyRef>,
    pub(super) operator_deploy: HashMap<String, OperatorRef>,
    pub(super) operator_undeploy: HashMap<String, OperatorRef>,
    pub(super) map: Map,
    pub(super) bullet_set: Vec<Bullet>,
    /// start for 1
    pub(super) next_id: usize,
    pub(super) kill_count: u32,
    pub(super) cost: f32,
    pub(super) life_point:i8,
    pub(super) events: Vec<Event>,
}

impl Frame {
    pub(super) fn step(&mut self, c: &mut Calculator) {
        self.event_step(c);
        self.map.update_enemy_map(self.enemy_set.clone());
        self.operator_step();
        self.enemy_step();
        self.bullet_step();
        self.cost += PERIOD;
    }
    fn operator_step(&mut self) {
        let ov = self.operator_deploy.clone();
        for (_key, o) in ov {
            let mut o = o.borrow_mut();
            o.next(self);
        }
    }
    fn enemy_step(&mut self) {
        let ve = self.enemy_set.clone();
        for e in ve {
            let mut eb = e.borrow_mut();
            eb.next(self);
        }
    }
    fn bullet_step(&mut self) {
        self.bullet_set.iter_mut().for_each(|b| b.step());
        let bv: Vec<Bullet> = self
            .bullet_set
            .iter()
            .filter(|&b| b.distance <= code::BULLET_HIT_DISTANCE)
            .cloned()
            .collect();
        for b in bv {
            let mut u = b.target.clone();
            u.be_hit(&b, self);
        }
        self.bullet_set
            .retain(|b| b.distance > code::BULLET_HIT_DISTANCE);
    }

    fn event_step(&mut self, c: &mut Calculator) {
        let ev = std::mem::take(&mut self.events);
        for e in ev {
            e.happen(self, c);
        }
    }

    pub(super) fn deep_clone(&self) -> Self {
        let mut enemy_set = Vec::<Rc<RefCell<Enemy>>>::new();
        for e in &self.enemy_set {
            enemy_set.push(Rc::new(RefCell::clone(&e)));
        }
        let mut operator_deploy = HashMap::<String, OperatorRef>::new();
        for (key, o) in self.operator_deploy.iter() {
            operator_deploy.insert(key.clone(), Rc::new(RefCell::new(o.borrow().deep_clone())));
        }
        let mut operator_undeploy = HashMap::<String, OperatorRef>::new();
        for (key, o) in self.operator_undeploy.iter() {
            operator_undeploy.insert(key.clone(), Rc::new(RefCell::new(o.borrow().deep_clone())));
        }
        let _bullet_set = self.bullet_set.clone();
        todo!();
        // Frame {
        //     timestamp: self.timestamp,
        //     enemy_set,
        //     operator_deploy,
        //     operator_undeploy,
        //     map: self.map.deep_clone(),
        //     bullet_set,
        //     next_id: self.next_id,
        //     ..Default::default()
        // }
    }

    pub(super) fn no_enemy(&self) -> bool {
        self.enemy_set.len() == 0
    }

    pub(super) fn get_obs(&self) -> serde_json::Value {
        let mut deploy = Vec::<String>::new();
        for (on, _) in self.operator_undeploy.iter() {
            // todo :add deploy time
            deploy.push(on.clone());
        }
        let mut retreat = Vec::<String>::new();
        for (on, _) in self.operator_deploy.iter() {
            retreat.push(on.clone());
        }
        let map = self.map.layout.clone();
        let v = json!({
            "Deploy":deploy,
            "Retreat":retreat,
            "Map":map,
        });
        v
    }

    pub(super) fn get_acs(&self) -> serde_json::Value {
        json!(&self)
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "\
time:{time}
num of enemy:{enemy_len}
cost:{cost}
enemy info:",
            time = self.timer,
            enemy_len = self.enemy_set.len(),
            cost = self.cost
        )?;
        for e in self.enemy_set.iter() {
            writeln!(f, "{}", e.borrow())?;
        }
        writeln!(f, "operators info:")?;
        for o in self.operator_deploy.values() {
            writeln!(f, "{}", o.borrow())?;
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for Frame {
    fn deserialize<D>(deserializer: D) -> Result<Frame, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = serde_json::Value::deserialize(deserializer)?;
        let map: Map = serde_json::from_value( v["map"].clone()).map_err(serde::de::Error::custom)?;
        let timer: Timer = serde_json::from_value(v["timer"].clone()).map_err(serde::de::Error::custom)?;
        let enemy_set: Vec<EnemyRef> = serde_json::from_value(v["enemy_set"].clone()).map_err(serde::de::Error::custom)?;
        let operator_deploy: HashMap<String, OperatorRef> = serde_json::from_value(v["operator_deploy"].clone()).map_err(serde::de::Error::custom)?;
        let operator_undeploy: HashMap<String, OperatorRef> = serde_json::from_value(v["operator_undeploy"].clone()).map_err(serde::de::Error::custom)?;
        let bullet_set: Vec<Bullet> = serde_json::from_value(v["bullet_set"].clone()).map_err(serde::de::Error::custom)?;
        let next_id: usize = serde_json::from_value(v["next_id"].clone()).map_err(serde::de::Error::custom)?;
        let kill_count: u32 = serde_json::from_value(v["kill_count"].clone()).map_err(serde::de::Error::custom)?;
        let cost: f32 = serde_json::from_value(v["cost"].clone()).map_err(serde::de::Error::custom)?;
        let life_point: i8 = serde_json::from_value(v["life_point"].clone()).map_err(serde::de::Error::custom)?;
        let events: Vec<Event> = serde_json::from_value(v["events"].clone()).map_err(serde::de::Error::custom)?;
        for i in 0..enemy_set.len() {
            let mut e = enemy_set[i].borrow_mut();
            let name = v["enemy_set"][i]["be_block"].as_str().unwrap();
            e.be_block = Rc::downgrade(&operator_deploy.get(name).unwrap());
        }
        for (key, o) in operator_deploy.iter() {
            let mut o = o.borrow_mut();
            let id = v["operator_deploy"][key]["be_block"].as_u64().unwrap() as usize;
            let e = enemy_set.iter().find(|e| e.borrow().id == id).unwrap();
            o.target = Rc::downgrade(e);
        }

        todo!("deserialize Frame");
        Ok(Frame {
            timer,
            enemy_set,
            operator_deploy,
            operator_undeploy,
            map,
            bullet_set,
            next_id,
            kill_count,
            cost,
            life_point,
            events,
        })
    }
}