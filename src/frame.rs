use std::cell::RefCell;
use std::collections::HashMap;
use crate::unit::enemy::Enemy;
use crate::unit::code;
use crate::unit::operator::Operator;
use crate::unit::bullet::Bullet;
use log::{info, trace};
use std::fmt;
use std::rc::Rc;
use crate::calculator::Calculator;
use crate::map;
#[derive(Debug,Clone)]
pub struct Frame {
    pub timestamp: u64,
    pub enemy_set: Vec<Rc<RefCell<Enemy>>>,
    pub operator_deploy:HashMap<String,Operator>,
    pub operator_undeploy:HashMap<String,Operator>,
    pub map:map::Map,
    pub bullet_set:Vec<Bullet>,
}

impl Frame {
    pub fn step(&mut self,c:&mut Calculator, t: f64) {
        for mut e in &self.enemy_set {
            let mut eb=e.borrow_mut();
            eb.step(t);
            if eb.die_code == code::INTO_END {
                info!("An enemy has enter to blue point");
            }
        }
        // self.enemy_set.iter_mut().for_each(|e| {
        //     e.calculate_vector();
        //     e.step(t);
        //     if (e.die_code == code::INTO_END) {
        //         info!("An enemy has enter to blue point");
        //     }
        // }
        // );
        self.enemy_set.retain(|e| e.borrow().die_code!=code::INTO_END);
        self.map.update_enemy_map(self.enemy_set.clone());
        for o in self.operator_deploy.iter_mut(){
            o.1.search(&self.map,self.timestamp);
        }
    }
    pub fn deep_clone(&self)->Self{
        let mut enemy_set=Vec::<Rc<RefCell<Enemy>>>::new();
        for e in &self.enemy_set{
            enemy_set.push(Rc::new(RefCell::clone(&e)));
        }
        let mut operator_deploy=HashMap::<String,Operator>::new();
        for (key,o) in self.operator_deploy.iter(){
            operator_deploy.insert(key.clone(),o.deep_clone());
        }
        let mut operator_undeploy=HashMap::<String,Operator>::new();
        for (key,o) in self.operator_undeploy.iter(){
            operator_undeploy.insert(key.clone(),o.deep_clone());
        }
        let mut bullet_set=self.bullet_set.clone();
        Frame{
            timestamp:self.timestamp,
            enemy_set,
            operator_deploy,
            operator_undeploy,
            map:self.map.deep_clone(),
            bullet_set,
        }
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "\
timestamp:{timestamp}
num of enemy:{enemy_len}
enemy info:",
            timestamp = self.timestamp,
            enemy_len = self.enemy_set.len()
        )?;
        for e in self.enemy_set.iter() {
            writeln!(f, "{}", e.borrow())?;
        }
        write!(f, "")
    }
}
