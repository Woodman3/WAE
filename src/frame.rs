use std::cell::RefCell;
use std::collections::HashMap;
use crate::unit::enemy::Enemy;
use crate::unit::code;
use crate::unit::operator::Operator;
use log::{info, trace};
use std::fmt;
use std::rc::Rc;
use crate::calculator::Calculator;
use crate::map;
#[derive(Debug)]
pub struct Frame {
    pub timestamp: u64,
    pub enemy_set: Vec<Rc<RefCell<Enemy>>>,
    pub operator_deploy:HashMap<String,Operator>,
    pub operator_undeploy:HashMap<String,Operator>,
    pub map:map::Map,
}

impl Frame {
    pub fn step(&mut self,c:&mut Calculator, t: f64) {
        for mut e in &self.enemy_set {
            let mut eb=e.borrow_mut();
            eb.calculate_vector();
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
impl Clone for Frame{
    fn clone(&self) -> Self {
        let mut enemy_set=Vec::<Rc<RefCell<Enemy>>>::new();
        for e in &self.enemy_set{
            enemy_set.push(Rc::new(RefCell::clone(&e)));
        }
        Frame{
            timestamp:self.timestamp,
            enemy_set,
            operator_deploy:self.operator_deploy.clone(),
            operator_undeploy:self.operator_undeploy.clone(),
            map:self.map.clone()
        }
    }
}
