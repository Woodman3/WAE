use std::cell::RefCell;
use super::scope::{Scope, Toward};
use std::fmt;
use std::ptr::write;
use std::rc::{Rc, Weak};
use log::{error, info, trace, warn};
use serde_json::Value;
use crate::calculator::PERIOD;
use crate::frame::Frame;
use crate::map::Map;
use crate::unit::bullet::Bullet;
use crate::unit::damage::Damage;
use crate::unit::enemy::{Enemy, EnemyWithPriority};
use crate::unit::Unit;
use crate::utils::math::{Grid, GridRect, Point};

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
    pub block_vec:Vec<Weak<RefCell<Enemy>>>,
    pub die_code: u32,
}

impl Operator {
    pub fn attack(&mut self,bv:&mut Vec<Bullet>){
        if let Some(e)=self.target.upgrade(){
            if self.stage.attack_time>0.0{
                self.stage.attack_time-=PERIOD;
            }else{
                match self.stage.attack_type.as_str() {
                    "Melee"=>{
                        let d=Damage{
                            value:self.stage.damage,
                            damage_type:self.stage.damage_type.clone(),
                        };
                        // self.target.upgrade().unwrap().borrow_mut().be_damage(&d);
                        e.borrow_mut().be_damage(&d);
                    }
                    "Ranged"=>{
                        bv.push(Bullet::new(
                            e,
                            Point::from(self.location),
                            2f64,
                            self.stage.damage_type.clone(),
                            self.stage.damage,
                        ));
                    }
                    _ => {error!("unknown attack_type!")}
                }
                self.stage.attack_time=self.info.attack_time;
            }
        }else{
            self.target=Weak::new();
        }
    }
    /// before call it,you should make sure that map haven't contain empty pointer
    pub fn search(&mut self,m:&Map,time_stamp:u64){
        let mut ve=Vec::<Weak<RefCell<Enemy>>>::new();
        for r in self.search_scope.0.iter(){
            for i in r.ul.row..=r.dr.row{
                for j in r.ul.col..=r.ul.col{
                    for e in m.enemy[i as usize][j as usize].iter(){
                        if let Some(e) =e.upgrade(){
                            if !ve.iter().any(|e2|{
                                if let Some(e2)=e2.upgrade(){
                                    e2==e
                                }else{
                                    false
                                }
                            }){
                                ve.push(Rc::downgrade(&e));
                            }
                        }
                        // if ve.iter().find(|&re|{
                        //     if let Some(e)=re.upgrade(){
                        //
                        //     }else{
                        //
                        //     }
                        // })==None{
                        //     ve.push(e.clone());
                        // }
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
    /// try to block enemy
    /// make sure all element in block_vec can be find
    pub fn block(&mut self,f:&mut Frame){
        let loc= self.location;
        for re in f.map.enemy[loc.row as usize][loc.col as usize].iter(){
            if let Some(re)=re.upgrade(){
                let mut e =re.borrow_mut();
                if e.stage.block_num<=self.stage.block_num{
                    e.be_block=Rc::downgrade(&f.operator_deploy[&self.name]);
                    info!("in {},{} block a enemy",f.timestamp,self.name);
                    self.block_vec.push(Rc::downgrade(&re));
                    self.stage.block_num-=e.stage.block_num;
                }
            }
        }
        self.block_vec.retain(|e|{
            if let Some(e)=e.upgrade(){
                true
            }else{
                false
            }
        });
    }
    pub fn next(&mut self,f:&mut Frame){
        // if let Some(e)=self.target.upgrade(){
        //     self.attack(&mut f.bullet_set);
        //     return
        // }
        self.block(f);
        if self.block_vec.len()!=0{
            self.target=self.block_vec[0].clone();
            self.attack(&mut f.bullet_set);
            return
        }
        self.search(&mut f.map,f.timestamp);
        if self.enemy_find.len()!=0{
            self.target=self.enemy_find[0].enemy.clone();
            self.attack(&mut f.bullet_set);
        }
    }
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
            block_vec:Vec::<Weak<RefCell<Enemy>>>::new(),
            die_code:0,
        })
    }

    pub fn deep_clone(&self)->Self{
        Operator{
            enemy_find:Vec::<EnemyWithPriority>::new(),
            target:Weak::new(),
            // block: self.block.clone(),//todo
            ..self.clone()
        }
    }
}

impl Unit for Operator {
    fn get_loc(&self) -> Point {
        Point::from(self.location)
    }

    fn be_hit(&mut self, b: &Bullet, f: &mut Frame) {
        self.be_damage(&b.damage);
        if self.stage.health<=0f64{
            self.die_code=super::code::DIE;
            trace!("an enemy has die!");
            return;
        }
    }


    fn be_damage(&mut self, d: &Damage) {
        match d.damage_type.as_str() {
            "Magic" =>{
                let damage=d.value*(1f64-self.stage.magic_resist);
                self.stage.health-=damage;
            }
            "Physical"=>{
                let damage=d.value-self.stage.armor;
                self.stage.health-=damage;
            }
            "Real"=>{
                self.stage.health-=d.value;
            }
            _ => {
                warn!("unknown attack type of bullet ,bullet has been departure");
                return
            }
            &_ => {}
        }
    }
}
