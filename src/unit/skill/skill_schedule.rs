use std::rc::Weak;
use crate::calculator::PERIOD;
use crate::frame::Frame;
use crate::unit::bullet::Bullet;
use crate::unit::operator::Operator;
use crate::unit::skill::{Skill, SkillEntity, ToEnemySkill};
use crate::unit::skill::effect::{Effect, FixedDamage};
use crate::unit::skill::effect::Effect::Damage;
use crate::unit::skill::skill_type::ChargeType;
use crate::unit::{skill, Unit};
use crate::utils::math::Point;

impl Operator{
    fn use_skill(&mut self, idx:usize, f:&mut Frame){
        use crate::unit::skill::skill_type::AttackType::*;
        match &self.skill_ready[idx].skill_entity {
            SkillEntity::ToEnemySkill(s) => {
                if let Some(u)=s.target.upgrade(){
                    match s.attack_type {
                        Melee=>{
                            let d= FixedDamage {
                                value:self.stage.atk,
                                damage_type:self.stage.damage_type.clone(),
                            };
                            u.borrow_mut().be_damage(&d);
                        }
                        Ranged=>{
                            f.bullet_set.push(Bullet::new(
                                u,
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
            SkillEntity::None => {}
        }
    }
    pub(crate) fn generate_default_attack_skill(&mut self){
        let mut s=Skill::default();
        s.duration=self.stage.attack_time;
        let d=super::effect::Damage{
            value:self.stage.atk,
            change:Option::None,
        };
        let se =ToEnemySkill{
            target:Weak::new(),
            effect:Damage(d),
            attack_type:self.stage.attack_type,
        };
        s.skill_entity=super::SkillEntity::ToEnemySkill(se);
        self.skill_ready.push(s);
    }
    fn skill(&mut self,f:&mut Frame){
        self.skill_block.retain_mut(|s| {
            if s.ready(){
                self.skill_ready.push(std::mem::take(s));
                return false
            }
            true
        });
        for s in self.skill_block.iter_mut(){
            if s.charge_type==ChargeType::Auto{
                s.charge(PERIOD);
            }
        }
        for i in 0..self.skill_ready.len(){
            if self.skill_ready[i].last>0.0{
                self.skill_ready[i].last-=PERIOD;
            }else{
                self.use_skill(i,f);
                self.skill_ready[i].last=0.0;
            }
        }
        self.skill_ready.retain_mut(|s|{
            if s.last==0.0{
                self.skill_block.push(std::mem::take(s));
                return false
            }
            true
        })
    }
}
