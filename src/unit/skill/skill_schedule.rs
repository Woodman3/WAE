use std::rc::Weak;
use crate::frame::Frame;
use crate::unit::bullet::Bullet;
use crate::unit::operator::Operator;
use crate::unit::skill::AttackSkill;
use crate::unit::skill::effect::Damage;
use crate::unit::Unit;
use crate::utils::math::Point;

impl Operator{
    pub fn use_skill(&mut self, s: AttackSkill, f:&mut Frame){
        use crate::unit::skill::skill_type::AttackType::*;
        if let Some(e)=s.target.upgrade(){
            match s.attack_type {
                Melee=>{
                    let d=Damage{
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
}