use std::cell::RefCell;
use super::scope::{Scope, Toward};
use std::fmt;
use std::ptr::write;
use std::rc::{Rc, Weak};
use log::info;
use serde_json::Value;
use crate::calculator::PERIOD;
use crate::frame::Frame;
use crate::map::Map;
use crate::unit::bullet::Bullet;
use crate::unit::enemy::{Enemy, EnemyWithPriority};
use crate::utils::math::{Grid, GridRect};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug,Clone)]
pub struct Operator{
    pub name: String,
    pub info:super::UnitInfo,
    pub stage:super::UnitInfo,
    pub location:Grid,
    pub attack_scope: Scope,
    pub search_scope: Scope,
    pub re_deploy:f32,
    pub toward:Toward,
    pub enemy_find:Vec<EnemyWithPriority>,
    pub target:Weak<RefCell<Enemy>>,
    pub block:Vec<Weak<RefCell<Enemy>>>,
}

impl Operator {
    pub fn new(v:&Value)->Result<Operator>{
        let t=serde_json::from_value::<Vec<Vec<i64>>>(v["attack_range"].clone())?;
        let mut t2 = Vec::<GridRect>::new();
        for ve in t {
            t2.push((GridRect::from((ve[0],ve[1],ve[2],ve[3]))));
        }
        let attack_scope= Scope(t2);
        let search_scope=attack_scope.clone();
        let info=serde_json::from_value::<super::UnitInfo>(v["UnitInfo"].clone())?;
        let stage=info.clone();
        Ok(Operator{
            name:serde_json::from_value(v["name"].clone())?,
            info,
            stage,
            location:(0,0).into(),
            attack_scope,
            search_scope,
            re_deploy:serde_json::from_value::<f32>(v["re_deploy"].clone())?,
            toward:Toward::East,
            enemy_find:Vec::<EnemyWithPriority>::new(),
            target:Weak::new(),
            block:vec![Weak::new()],
        })
    }
    pub fn attack(&mut self,bv:&mut Vec<Bullet>){
        if self.stage.attack_time>0.0{
            self.stage.attack_time-=PERIOD;
        }else{
            bv.push(Bullet::new(
                self.target.upgrade().unwrap(),
                self.location.into(),
                2f64,
                self.stage.damage_type.clone(),
                self.stage.damage,
            ));
            self.stage.attack_time=self.info.attack_time;
        }
    }
    pub fn search(&mut self,m:&Map,time_stamp:u64){
        let mut ve=Vec::<Rc<RefCell<Enemy>>>::new();
        for r in self.search_scope.0.iter(){
            for i in r.ul.row..=r.dr.row{
                for j in r.ul.col..=r.ul.col{
                    for e in m.enemy[i as usize][j as usize].iter(){
                        if ve.iter().find(|&re| re==e)==None{
                            ve.push(Rc::clone(e));
                        }
                    }
                }
            }
        }
        let mut c=0;
        for e in ve{
            self.enemy_find.push(EnemyWithPriority{enemy:e,time_stamp});
            c+=1;
        }
        info!("in {time_stamp},{} search {c} enemy",self.name);
    }
    pub fn block(&mut self,f:&mut Frame){
        let loc:Grid = self.location.into();
        for re in f.map.enemy[loc.row as usize][loc.col as usize].iter(){
            let e =re.borrow();
            if e.stage.block_num<=self.stage.block_num{
                self.block.push(Rc::downgrade(re));
                self.stage.block_num-=e.stage.block_num;
            }
        }
    }

    pub fn deep_clone(&self)->Self{
        Operator{
            name:self.name.clone(),
            info:self.info.clone(),
            stage:self.stage.clone(),
            location:self.location,
            attack_scope:self.attack_scope.clone(),
            search_scope:self.search_scope.clone(),
            re_deploy:self.re_deploy,
            toward:self.toward.clone(),
            enemy_find:Vec::<EnemyWithPriority>::new(),
            target:Weak::new(),
            block: self.block.clone(),//todo
        }
    }
}
