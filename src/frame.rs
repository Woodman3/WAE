use std::cell::RefCell;
use std::collections::HashMap;
use crate::unit::enemy::Enemy;
use crate::unit::code;
use crate::unit::operator::Operator;
use crate::unit::bullet::Bullet;
use log::info;
use serde_json::json;
use std::fmt;
use std::rc::Rc;
use crate::calculator::Calculator;
use crate::map;

pub type OperatorRef =Rc<RefCell<Operator>>;
#[derive(Debug,Clone)]
pub struct Frame {
    pub timestamp: u64,
    pub enemy_set: Vec<Rc<RefCell<Enemy>>>,
    pub operator_deploy:HashMap<String,OperatorRef>,
    pub operator_undeploy:HashMap<String,OperatorRef>,
    pub map:map::Map,
    pub bullet_set:Vec<Bullet>,
    pub next_id:usize,
}

impl Frame {
    pub fn step(&mut self,_c:&mut Calculator) {
        self.map.update_enemy_map(self.enemy_set.clone());
        self.operator_step();
        self.enemy_step();
        self.bullet_step();
        self.enemy_set.retain(|e| e.borrow().die_code!=code::DIE);
    }
    fn operator_step(&mut self){
        let ov=self.operator_deploy.clone();
        for (_key,o) in ov{
            let mut o=o.borrow_mut();
            o.next(self);
        }
    }
    fn enemy_step(&mut self){
        self.enemy_set.retain(|e| e.borrow().die_code!=code::INTO_END);
        let ve=self.enemy_set.clone();
        for e in ve{
            let mut eb=e.borrow_mut();
            eb.next(self);
                if eb.die_code == code::INTO_END {
                    info!("An enemy has enter to blue point");
                }
        }
    }
    fn bullet_step(&mut self){
        self.bullet_set.iter_mut().for_each(|b| b.step());
        let bv:Vec<Bullet>=self.bullet_set.iter().filter(|&b| b.distance<=code::BULLET_HIT_DISTANCE).cloned().collect();
        for b in bv{
            let mut u=b.target.borrow_mut();
            u.be_hit(&b,self);
        }
        self.bullet_set.retain(|b| b.distance>code::BULLET_HIT_DISTANCE);
    }
    // Todo
    pub fn deep_clone(&self)->Self{
        let mut enemy_set=Vec::<Rc<RefCell<Enemy>>>::new();
        for e in &self.enemy_set{
            enemy_set.push(Rc::new(RefCell::clone(&e)));
        }
        let mut operator_deploy=HashMap::<String,OperatorRef>::new();
        for (key,o) in self.operator_deploy.iter(){
            operator_deploy.insert(key.clone(),Rc::new(RefCell::new(o.borrow().deep_clone())));
        }
        let mut operator_undeploy=HashMap::<String,OperatorRef>::new();
        for (key,o) in self.operator_undeploy.iter(){
            operator_undeploy.insert(key.clone(),Rc::new(RefCell::new(o.borrow().deep_clone())));
        }
        let bullet_set=self.bullet_set.clone();
        Frame{
            timestamp:self.timestamp,
            enemy_set,
            operator_deploy,
            operator_undeploy,
            map:self.map.deep_clone(),
            bullet_set,
            next_id:self.next_id,
        }
    }

    pub fn no_enemy(&self)->bool{
        self.enemy_set.len()==0
    }

    pub(super) fn get_obs(&self)->serde_json::Value{
        let mut deploy  = Vec::<String>::new();
        for (on,_) in self.operator_undeploy.iter(){
            // todo :add deploy time
            deploy.push(on.clone());
        }
        let mut retreat = Vec::<String>::new(); 
        for (on,_) in self.operator_deploy.iter(){
            retreat.push(on.clone());
        }
        let map = self.map.layout.clone();
        let v=json!({
            "Deploy":deploy,
            "Retreat":retreat,
            "Map":map,
        });
        v
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
