use std::cell::RefCell;
use std::rc::{Rc, Weak};
use eframe::egui::Key::P;
use crate::calculator::PERIOD;
use crate::frame::Frame;
use crate::map::Map;
use crate::unit::skill::skill_type::{ChargeType,TriggerType,AttackType};
use crate::unit::skill::effect::{FixedDamage, Effect};
use crate::unit::skill::ToEnemySkill;
use crate::unit::bullet::Bullet;
use crate::unit::enemy::{Enemy, EnemyWithPriority};
use crate::unit::operator::Operator;
use crate::unit::skill::effect::{ChangeClass, ChangeType};
use crate::unit::Unit;
use crate::utils::math::Point;
use super::super::skill::effect::Buff;

impl Operator{
    pub fn attack(&mut self,f:&mut Frame){
        if let Some(e)=self.target.upgrade(){
            use super::super::skill::skill_type::AttackType::*;
            match self.stage.attack_type {
                Melee=>{
                    let d= FixedDamage {
                        value:self.stage.atk,
                        damage_type:self.stage.damage_type.clone(),
                    };
                    e.borrow_mut().be_damage(&d);
                }
                Ranged=>{
                    f.bullet_set.push(Bullet::new(
                        e,
                        Point::from(self.location),
                        2f64,
                        self.stage.damage_type,
                        self.stage.atk,
                    ));
                }
                _ => { log::error!("unknown attack_type!")}
            }
        }else{
            self.target=Weak::new();
            self.stage.attack_time=self.info.attack_time;
        }

    }
    // pub fn attack_skill(&mut self, f:&mut Frame){
    //     if let Some(skill) = &mut self.skill{
    //         if self.stage.attack_time>0.0{
    //             self.stage.attack_time-=PERIOD;
    //         }else{
    //             for eff in skill.effect.clone().into_iter(){
    //                 self.be_effect(eff);
    //             }
    //             self.attack(f);
    //             self.stage.attack_time=self.info.attack_time;
    //         }
    //     }else{
    //         self.target=Weak::new();
    //         self.stage.attack_time=self.info.attack_time;
    //     }
    // }
    pub fn attack_mission(&mut self,f:&mut Frame){
        // if let Some(skill) = &mut self.skill {
        //     if skill.ready(){
        //         self.skill(f);
        //         return;
        //     }else{
        //         if skill.charge_type==Auto{
        //             skill.charge(PERIOD);
        //         }
        //     }
        // }
        if self.stage.attack_time>0.0{
            self.stage.attack_time-=PERIOD;
        }else{
            self.stage.attack_time=self.info.attack_time;
            self.attack(f);
        }
    }

    /// before call it,you should make sure that map haven't contain empty pointer
    pub fn search(&mut self,m:&Map,time_stamp:u64){
        self.enemy_find.clear();
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
                    }

                }
            }
        }
        for e in ve{
            self.enemy_find.push(EnemyWithPriority{enemy:e,time_stamp});
        }
    }
    /// try to block enemy
    /// make sure all element in block_vec can be find
    pub fn block(&mut self,f:&mut Frame){
        self.block_vec.retain(|e|{
            if let Some(e)=e.upgrade(){
                true
            }else{
                false
            }
        });
        let loc= self.location;
        for re in f.map.enemy[loc.row as usize][loc.col as usize].iter(){
            if let Some(re)=re.upgrade(){
                if !self.block_vec.iter().any(|e2| {
                    if let Some(e2) = e2.upgrade(){
                        e2==re
                    }else{
                        false
                    }
                }){
                    let mut e =re.borrow_mut();
                    if e.stage.block_num<=self.stage.block_num{
                        e.be_block=Rc::downgrade(&f.operator_deploy[&self.name]);
                        self.block_vec.push(Rc::downgrade(&re));
                        self.stage.block_num-=e.stage.block_num;
                    }
                }
            }
        }
    }
    pub fn get_target(&mut self,f:&mut Frame){
        if self.block_vec.len()!=0{
            self.target=self.block_vec[0].clone();
            return
        }
        self.search(&mut f.map,f.timestamp);
        if self.enemy_find.len()!=0{
            self.target=self.enemy_find[0].enemy.clone();
        }
    }
    fn be_effect(&mut self,e:Effect){
       match e{
           Effect::Buff(b) => {
               self.stage.be_buff(b);
           }
           Effect::FixedDamage(d) => {
               self.be_damage(&d);
           }
           _ => {}
       }
    }
}