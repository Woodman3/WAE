use log::error;

use crate::calculator::PERIOD;
use crate::frame::Frame;
use crate::map::Map;
use crate::unit::bullet::Bullet;
use crate::unit::skill::{Skill, SkillEntity, ToEnemySkill};
use crate::unit::Unit;
use crate::utils::math::Point;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use super::ToOperatorSkill;

impl ToEnemySkill {
    pub(crate) fn search(&mut self, m: &Map) -> bool {
        if let Some(s) = &self.search_scope {
            let mut ve = m.search_enemy(s);
            if ve.len() >= self.target_num {
                self.target = ve.drain(0..self.target_num).collect();
            } else {
                self.target = ve.drain(..).collect();
            }
        }
        self.target.len() != 0
    }
}

impl ToOperatorSkill {
    pub(crate) fn search(&mut self, m: &Map) -> bool {
        if let Some(s) = &self.search_scope {
            let mut vo = m.search_operator(s);
            if vo.len() >= self.target_num {
                self.target = vo.drain(0..self.target_num).collect();
            } else {
                self.target = vo.drain(..).collect();
            }
        }
        self.target.len() != 0
    }
}

impl Display for ToEnemySkill {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\
        target_find{}\n\
        ",
            self.target.len()
        )
    }
}

impl Display for Skill {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\
        duration:{}\n\
        last:{}\n\
        ",
            self.duration, self.last
        );
        match &self.skill_entity {
            SkillEntity::ToEnemySkill(se) => {
                write!(f, "{}\n", se)
            }
            _ => {
                write!(f, "")
            }
        }
    }
}

impl Skill {
    pub(super) fn ready(&self) -> bool {
        self.last != 0.0 && self.sp_data.sp >= self.sp_data.sp_cost
    }
    pub(super) fn can_charge(&self) -> bool {
        self.sp_data.sp < self.sp_data.sp_cost || self.sp_data.overcharge
    }
    pub(super) fn charge(&mut self, value: f64) {
        if self.can_charge() {
            self.sp_data.sp += value
        }
    }
    pub(super) fn can_run(&mut self, f: &Frame) -> bool {
        match &mut self.skill_entity {
            SkillEntity::ToEnemySkill(s) => {
                if s.search(&f.map) {
                    return true;
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    pub(super) fn step(&mut self, f: &mut Frame) -> bool {
        self.last -= PERIOD;
        if self.last <= 0.0 {
            self.shoot(f);
            self.last = self.duration;
            return true;
        }
        false
    }
    pub(super) fn shoot(&self, f: &mut Frame) {
        match &self.skill_entity {
            SkillEntity::ToEnemySkill(s) => {
                for t in s.target.iter() {
                    if let Some(u) = t.upgrade() {
                        use crate::unit::skill::skill_type::AttackType::*;
                        match s.attack_type {
                            Melee => {
                                u.borrow_mut().be_effect(&s.effect);
                            }
                            Ranged => {
                                // f.bullet_set.push(Bullet::new(
                                //     Unit::Enemy(Rc::clone(&u)),
                                //     Point::from(self.location),
                                //     2f64,
                                //     self.stage.damage_type,
                                //     self.stage.atk,
                                // ));
                                todo!("ranged skill");
                            }
                            _ => {
                                todo!("unknown attack type of enemy");
                            }
                        }
                    } else {
                        error!("target not found!");
                    }
                }
            }
            SkillEntity::ToOperatorSkill(s) => {
                todo!()
            }
            SkillEntity::None => {}
        }
    }
}
