use std::rc::Weak;
use crate::frame::Frame;
use crate::unit::bullet::Bullet;
use crate::unit::operator::Operator;
use crate::unit::skill::{Skill, ToEnemySkill};
use crate::unit::skill::effect::{Effect, FixedDamage};
use crate::unit::skill::effect::Effect::Damage;
use crate::unit::skill::skill_type::ChargeType;
use crate::unit::{skill, Unit};
use crate::utils::math::Point;

impl Operator{
    fn use_skill(&mut self, s:ToEnemySkill, f:&mut Frame){
        use crate::unit::skill::skill_type::AttackType::*;
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
        self.skill.push(s);
    }
    fn skill(&mut self){
    }
}