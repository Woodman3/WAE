use crate::map::Map;
use crate::unit::skill::{Skill, SkillEntity, ToEnemySkill};
use std::fmt::{Display, Formatter};

impl ToEnemySkill {
    pub(crate) fn search(&mut self, m: &Map) -> bool {
        if let Some(s) = &self.search_scope {
            let mut ve = m.search(s);
            if ve.len() >= self.target_num {
                self.target = ve.drain(0..self.target_num).collect();
            } else {
                self.target = ve.drain(..).collect();
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
            SkillEntity::None => {
                write!(f, "")
            }
        }
    }
}
