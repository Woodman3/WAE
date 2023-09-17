use std::cell::RefCell;
use super::scope::{Scope, Toward};
use std::fmt;
use std::rc::Rc;
use log::info;
use serde_json::Value;
use crate::map::Map;
use crate::unit::enemy::{Enemy, EnemyWithPriority};
use crate::utils::math::GridRect;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug,Clone)]
pub struct Operator{
    info:super::UnitInfo,
    pub location:(u32,u32),
    pub attack_scope: Scope,
    pub search_scope: Scope,
    pub re_deploy:f32,
    pub toward:Toward,
    enemy_find:Vec<EnemyWithPriority>
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
        Ok(Operator{
            info: serde_json::from_value::<super::UnitInfo>(v["UnitInfo"].clone())?,
            location:(0,0),
            attack_scope,
            search_scope,
            re_deploy:serde_json::from_value::<f32>(v["re_deploy"].clone())?,
            toward:Toward::East,
            enemy_find:Vec::<EnemyWithPriority>::new()
        })
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
                        info!("in {time_stamp},we have search a enemy in :{:?}",e.borrow().location)
                    }
                }
            }
        }
    }
    pub fn deep_clone(&self)->Self{
        Operator{
            info:self.info.clone(),
            location:self.location,
            attack_scope:self.attack_scope.clone(),
            search_scope:self.search_scope.clone(),
            re_deploy:self.re_deploy,
            toward:self.toward.clone(),
            enemy_find:Vec::<EnemyWithPriority>::new()
        }
    }
}
