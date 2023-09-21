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
use crate::utils::math::GridRect;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug,Clone)]
pub struct Operator{
    info:super::UnitInfo,
    stage:super::UnitStage,
    pub location:(u32,u32),
    pub attack_scope: Scope,
    pub search_scope: Scope,
    pub re_deploy:f32,
    pub toward:Toward,
    pub enemy_find:Vec<EnemyWithPriority>,
    pub target:Weak<RefCell<Enemy>>
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
        let stage:super::UnitStage=info.clone().into();
        Ok(Operator{
            info,
            stage,
            location:(0,0),
            attack_scope,
            search_scope,
            re_deploy:serde_json::from_value::<f32>(v["re_deploy"].clone())?,
            toward:Toward::East,
            enemy_find:Vec::<EnemyWithPriority>::new(),
            target:Weak::new(),
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
                self.stage.attack_type.clone(),
                self.stage.damage,
            ));
            self.stage.attack_time=self.info.attack_time;
        }
    }
    pub fn search(&mut self,m:&Map,time_stamp:u64){
        for r in self.search_scope.0.iter(){
            for i in r.ul.row..=r.dr.row{
                for j in r.ul.col..=r.ul.col{
                    for e in m.enemy[i as usize][j as usize].iter(){
                        self.enemy_find.push(
                            EnemyWithPriority{
                                enemy:Rc::clone(&e),
                                time_stamp
                            }
                        );
                        info!("in {time_stamp},we have search a enemy in :({i},{j}),it's detail location is {:?} ",e.borrow().location)
                    }
                }
            }
        }
    }

    pub fn deep_clone(&self)->Self{
        Operator{
            info:self.info.clone(),
            stage:self.stage.clone(),
            location:self.location,
            attack_scope:self.attack_scope.clone(),
            search_scope:self.search_scope.clone(),
            re_deploy:self.re_deploy,
            toward:self.toward.clone(),
            enemy_find:Vec::<EnemyWithPriority>::new(),
            target:Weak::new(),
        }
    }
}
