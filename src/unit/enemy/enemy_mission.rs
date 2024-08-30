use std::{cell::RefCell, rc::Weak};

use crate::{frame::Frame, unit::skill::{effect, Skill, SkillEntity, ToOperatorSkill}};

use super::Enemy;

impl Enemy{
    fn skill(&mut self,f:&mut Frame){
        self.skills.step(f);
    }
    fn generate_default_attack_skill(&mut self) {
        let mut s = Skill::default();
        s.duration = self.stage.attack_time;
        let d = effect::FixedDamage {
            value: self.stage.atk,
            damage_type: self.stage.damage_type.clone(),
        };
        let se = ToOperatorSkill {
            host : self.self_weak.clone(),
            target: Vec::new(),
            target_num: 1,
            effect: effect::Effect::FixedDamage(d),
            attack_type: self.stage.attack_type,
            search_scope: Option::from(self.stage.scope.clone()),
        };
        s.skill_entity = SkillEntity::ToOperatorSkill(se);
        self.skills.skill_block.push(s);
    }
}