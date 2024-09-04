use serde::{Deserialize, Serialize};

use crate::calculator::PERIOD;
use crate::frame::Frame;
use crate::unit::skill::{Skill};
use crate::unit::Unit;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug, Serialize)]
#[serde(default)]
pub(crate) struct SkillSchedule {
    pub(crate) skill_block: Vec<Skill>,
    pub(crate) skill_ready: Vec<Skill>,
    pub(crate) skill_running: Vec<Skill>,
    pub(crate) host: Unit,
}

impl SkillSchedule {
    pub(crate) fn step(&mut self, f: &mut Frame) {
        self.skill_block.retain_mut(|s| {
            s.charge(PERIOD);
            if s.ready() {
                self.skill_ready.push(std::mem::take(s));
                false
            } else {
                true
            }
        });
        self.skill_ready.retain_mut(|s| {
            if s.can_run(f) {
                self.skill_running.push(std::mem::take(s));
                false
            } else {
                true
            }
        });
        self.skill_running.retain_mut(|s| {
            if s.step(f,&self.host) {
                self.skill_block.push(std::mem::take(s));
                false
            } else {
                true
            }
        });
    }
}

impl Display for SkillSchedule {
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
