use std::{cell::RefCell, rc::Weak};

use crate::{frame::Frame, unit::skill::{effect, Skill, ToOperatorSkill}};

use super::Enemy;

impl Enemy{
    fn skill(&mut self,f:&mut Frame){
        self.skills.step(f);
    }
    // fn generate_default_attack_skill(&mut self) {
    //     let mut s = Skill::default();
    //     s.duration = self.stage.attack_time;
    //     let d = effect::Damage {
    //         value: self.stage.atk,
    //         change: Option::None,
    //     };
    //     let se = ToOperatorSkill {
    //         target: Vec::<Weak<RefCell<Enemy>>>::new(),
    //         target_num: 1,
    //         effect: effect::Effect::Damage(d),
    //         attack_type: self.stage.attack_type,
    //         search_scope: Option::from(self.search_scope.clone()),
    //     };
    //     s.skill_entity = SkillEntity::ToEnemySkill(se);
    //     self.skills.skill_block.push(s);
    // }
}