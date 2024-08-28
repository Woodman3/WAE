use serde::{Deserialize, Serialize};

use crate::calculator::PERIOD;
use crate::frame::Frame;
use crate::unit::bullet::Bullet;
use crate::unit::enemy::Enemy;
use crate::unit::operator::Operator;
use crate::unit::skill::effect::Effect::Damage;
use crate::unit::skill::effect::FixedDamage;
use crate::unit::skill::skill_type::ChargeType;
use crate::unit::skill::{Skill, SkillEntity, ToEnemySkill};
use crate::unit::Unit;
use crate::utils::math::Point;
use std::cell::RefCell;
use std::fmt::{self, Display, Formatter};
use std::rc::{Rc, Weak};

#[derive(Clone, Deserialize, Debug, Default, Serialize)]
#[serde(default)]
pub(crate) struct SkillSchedule{
    pub(crate) skill_block:Vec<Skill>,
    pub(crate) skill_ready:Vec<Skill>,
    pub(crate) skill_running:Vec<Skill>,
}

impl Operator {
    // fn shoot(&mut self, idx: usize, f: &mut Frame) {
    //     match &self.skill_ready[idx].skill_entity {
    //         SkillEntity::ToEnemySkill(s) => {
    //             for t in s.target.iter() {
    //                 if let Some(u) = t.upgrade() {
    //                     use crate::unit::skill::skill_type::AttackType::*;
    //                     match s.attack_type {
    //                         Melee => {
    //                             let d = FixedDamage {
    //                                 value: self.stage.atk,
    //                                 damage_type: self.stage.damage_type.clone(),
    //                             };
    //                             u.borrow_mut().be_damage(&d);
    //                         }
    //                         Ranged => {
    //                             f.bullet_set.push(Bullet::new(
    //                                 Unit::Enemy(Rc::clone(&u)),
    //                                 Point::from(self.location),
    //                                 2f64,
    //                                 self.stage.damage_type,
    //                                 self.stage.atk,
    //                             ));
    //                         }
    //                         _ => {
    //                             todo!("unknown attack type of enemy");
    //                         }
    //                     }
    //                 } else {
    //                     self.target = Weak::new();
    //                     self.stage.attack_time = self.info.attack_time;
    //                 }
    //             }
    //         }
    //         SkillEntity::None => {}
    //     }
    // }
    pub(crate) fn generate_default_attack_skill(&mut self) {
        let mut s = Skill::default();
        s.duration = self.stage.attack_time;
        let d = super::effect::Damage {
            value: self.stage.atk,
            change: Option::None,
        };
        let se = ToEnemySkill {
            target: Vec::<Weak<RefCell<Enemy>>>::new(),
            target_num: 1,
            effect: Damage(d),
            attack_type: self.stage.attack_type,
            search_scope: Option::from(self.search_scope.clone()),
        };
        s.skill_entity = super::SkillEntity::ToEnemySkill(se);
        self.skills.skill_block.push(s);
    }
    // pub(crate) fn skill(&mut self, f: &mut Frame) {
    //     self.skill_block.retain_mut(|s| {
    //         if s.ready() {
    //             self.skill_ready.push(std::mem::take(s));
    //             return false;
    //         }
    //         true
    //     });
    //     for s in self.skill_block.iter_mut() {
    //         if s.sp_data.charge_type == ChargeType::Time {
    //             s.charge(PERIOD);
    //         }
    //     }
    //     for i in 0..self.skill_ready.len() {
    //         match &mut self.skill_ready[i].skill_entity {
    //             SkillEntity::ToEnemySkill(s) => {
    //                 if s.search(&f.map) {
    //                     if self.skill_ready[i].last > 0.0 {
    //                         self.skill_ready[i].last -= PERIOD;
    //                     } else {
    //                         self.shoot(i, f);
    //                         self.skill_ready[i].last = 0.0;
    //                     }
    //                 } else {
    //                     self.skill_ready[i].last = self.skill_ready[i].duration;
    //                 }
    //             }
    //             SkillEntity::None => {}
    //         }
    //     }
    //     self.skill_ready.retain_mut(|s| {
    //         if s.last == 0.0 {
    //             s.last = s.duration;
    //             self.skill_block.push(std::mem::take(s));
    //             return false;
    //         }
    //         true
    //     })
    // }
}

impl SkillSchedule{
    fn step(&mut self,f:&mut Frame){
        self.skill_block.retain_mut(|s|{
            s.charge(PERIOD);
            if s.ready(){
                self.skill_ready.push(std::mem::take(s));
                false
            }else{
                true
            }
        });
        self.skill_ready.retain_mut(|s|{
            if s.can_run(f){
                self.skill_running.push(std::mem::take(s));
                false
            }else{
                true
            }
        });
        self.skill_running.retain_mut(|s|{
            if s.step(f){
                self.skill_block.push(std::mem::take(s));
                false
            }else{
                true
            }
        });
    }
}

impl Display for SkillSchedule{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
        skill_block:{}\n\
        skill_ready:{}\n\
        skill_running:{}\n\
        ",
            self.skill_block.len(),
            self.skill_ready.len(),
            self.skill_running.len()
        )
    }
}