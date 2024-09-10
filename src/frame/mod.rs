use crate::calculator::{Calculator, PERIOD};
use crate::map;
use crate::event::Event;
use crate::unit::bullet::Bullet;
use crate::unit::{code, skill};
use crate::unit::enemy::Enemy;
use crate::unit::operator::Operator;
use crate::unit::skill::skill_schedule::SkillSchedule;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, write};
use std::rc::Rc;

pub(super) type OperatorRef = Rc<RefCell<Operator>>;
pub(super) type EnemyRef = Rc<RefCell<Enemy>>;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub(super) struct Frame {
    pub(super) timestamp: u64,
    pub(super) enemy_set: Vec<EnemyRef>,
    pub(super) operator_deploy: HashMap<String, OperatorRef>,
    pub(super) operator_undeploy: HashMap<String, OperatorRef>,
    pub(super) map: map::Map,
    pub(super) bullet_set: Vec<Bullet>,
    /// start for 1
    pub(super) next_id: usize,
    pub(super) kill_count: u32,
    pub(super) cost: f32,
    pub(super) events: Vec<Event>,
}

impl Frame {
    pub(super) fn step(&mut self, c: &mut Calculator) {
        self.event_step(c);
        self.map.update_enemy_map(self.enemy_set.clone());
        self.operator_step();
        self.enemy_step();
        self.bullet_step();
        self.cost += PERIOD as f32;
    }
    fn operator_step(&mut self) {
        let ov = self.operator_deploy.clone();
        for (_key, o) in ov {
            let mut o = o.borrow_mut();
            o.next(self);
        }
    }
    fn enemy_step(&mut self) {
        self.enemy_set
            .retain(|e| e.borrow().die_code != code::INTO_END);
        let ve = self.enemy_set.clone();
        for e in ve {
            let mut eb = e.borrow_mut();
            eb.next(self);
            if eb.die_code == code::INTO_END {
                self.kill_count += 1;
                info!("An enemy has enter to blue point");
            }
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

    fn event_step(&mut self,c: &mut Calculator) {
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
timestamp:{timestamp}
time:{time}
num of enemy:{enemy_len}
cost:{cost}
enemy info:",
            timestamp = self.timestamp,
            time = self.timestamp as f64 * PERIOD,
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
