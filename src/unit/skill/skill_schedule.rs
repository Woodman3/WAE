use serde::{Deserialize, Serialize};

use crate::calculator::PERIOD;
use crate::frame::Frame;
use crate::unit::skill::Skill;
use std::fmt::{self, Display, Formatter};

use super::ChargeType;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub(crate) struct SkillSchedule {
    pub(crate) skill_block: Vec<Skill>,
    pub(crate) skill_ready: Vec<Skill>,
    pub(crate) skill_running: Vec<Skill>,
    pub(crate) message_buffer: Vec<SkillMessage>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Copy)]
pub(crate) enum SkillMessage {
    Attack,
}

impl SkillSchedule {
    pub(crate) fn step(&mut self, f: &mut Frame) -> Vec<Skill> {
        let mut r = vec![];
        self.skill_block.retain_mut(|s| {
            match s.sp_data.charge_type {
                ChargeType::Time => s.charge(PERIOD),
                ChargeType::Attack => {
                    for m in self.message_buffer.iter() {
                        if matches!(m, SkillMessage::Attack) {
                            s.charge(1.0);
                        }
                    }
                }
                ChargeType::BeHit => todo!(),
                ChargeType::None => {}
            }
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
            if s.step(f) {
                r.push(std::mem::take(s));
                false
            } else {
                true
            }
        });
        self.message_buffer.clear();
        r
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
