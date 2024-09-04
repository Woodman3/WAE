use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use std::cell::{Ref, RefCell};
use std::path::Path;
use std::rc::Rc;

use super::data_loader::Loader;
use super::load_json_file;
use super::math::Grid;
use crate::calculator::Calculator;
use crate::frame::Frame;
use crate::timeline::doctor::{
    OperatorDeployEvent, OperatorRetreatEvent, OperatorSkillEvent, UnitRetreatEvent, UnitSkillEvent,
};
use crate::timeline::Event;
use crate::unit::operator::OperatorShared;
use crate::unit::scope::Toward;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug, Default)]
pub(crate) struct Copilot {
    copilot_data: CopilotData,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub(super) struct CopilotData {
    pub(super) stage_name: String,
    #[serde(rename = "opers")]
    pub(super) operators: Vec<CopilotOperator>,
    pub(super) groups: Vec<CopilotGroup>,
    pub(super) actions: Vec<CopilotAction>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub(super) struct CopilotOperator {
    pub(super) name: String,
    pub(super) skill: Option<u8>,
    pub(super) skill_usage: Option<u8>,
    pub(super) skill_times: Option<u8>,
    pub(super) requirement: Option<CopilotRequirement>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub(super) struct CopilotRequirement {
    pub(super) elite: Option<u8>,
    pub(super) level: Option<u8>,
    pub(super) skill_level: Option<u8>,
    pub(super) module: Option<u8>,
    pub(super) potential: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub(super) struct CopilotGroup {
    pub(super) name: String,
    #[serde(rename = "opers")]
    pub(super) operators: Vec<CopilotOperator>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub(super) enum CopilotAction {
    Deploy(CopilotActionDeploy),
    Skill(CopilotActionSkill),
    Retreat(CopilotActionRetreat),
    SkillDaemon,
    #[serde(other)]
    UseLess,
}

//todo: may be all condition can warp into a single struct?
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub(super) struct CopilotActionDeploy {
    pub(super) name: String,
    pub(super) location: Grid,
    pub(super) direction: Toward,
    #[serde(flatten)]
    pub(super) condition: Option<CopilotActionCondition>,
    pub(super) pre_delay: Option<u8>,
    pub(super) post_delay: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub(super) struct CopilotActionCondition {
    pub(super) kills: Option<u8>,
    pub(super) costs: Option<u8>,
    pub(super) cost_changes: Option<u8>,
    pub(super) cooling: Option<i8>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub(super) struct CopilotActionSkill {
    pub(super) name: Option<String>,
    pub(super) skill_usage: Option<u8>,
    pub(super) skill_times: Option<u8>,
    pub(super) location: Option<Grid>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub(super) struct CopilotActionRetreat {
    pub(super) name: Option<String>,
    pub(super) location: Option<Grid>,
}

impl Into<OperatorDeployEvent> for CopilotActionDeploy {
    fn into(self) -> OperatorDeployEvent {
        OperatorDeployEvent {
            operator_key: self.name.clone(),
            location: self.location,
            toward: self.direction,
        }
    }
}

impl TryInto<OperatorRetreatEvent> for CopilotActionRetreat {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<OperatorRetreatEvent> {
        let operator_key = self.name.clone().ok_or("without operator name")?;
        Ok(OperatorRetreatEvent { operator_key })
    }
}

impl TryInto<OperatorSkillEvent> for CopilotActionSkill {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<OperatorSkillEvent> {
        let operator_key = self.name.clone().ok_or("without operator name")?;
        Ok(OperatorSkillEvent { operator_key })
    }
}

impl TryInto<UnitSkillEvent> for CopilotActionSkill {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<UnitSkillEvent> {
        let location = self.location.ok_or("without location")?;
        Ok(UnitSkillEvent { location })
    }
}

impl TryInto<UnitRetreatEvent> for CopilotActionRetreat {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<UnitRetreatEvent> {
        let location = self.location.ok_or("without location")?;
        Ok(UnitRetreatEvent { location })
    }
}

impl TryInto<Event> for CopilotAction {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<Event> {
        match self {
            CopilotAction::Deploy(d) => Ok(Event::OperatorDeployEvent(d.into())),
            CopilotAction::Skill(s) => {
                if s.name.is_some() {
                    Ok(Event::OperatorSkillEvent(s.try_into()?))
                } else {
                    Ok(Event::UnitSkillEvent(s.try_into()?))
                }
            }
            CopilotAction::Retreat(r) => {
                if r.name.is_some() {
                    Ok(Event::OperatorRetreatEvent(r.try_into()?))
                } else {
                    Ok(Event::UnitRetreatEvent(r.try_into()?))
                }
            }
            CopilotAction::SkillDaemon => todo!("skill daemon"),
            CopilotAction::UseLess => Err("useless action".into()),
        }
    }
}

impl CopilotAction {
    fn check(&self, f: &Frame) -> bool {
        match self {
            CopilotAction::Deploy(d) => {
                if !f.operator_undeploy.contains_key(&d.name) {
                    return false;
                }
                d.condition.as_ref().map_or(true, |c| c.check(f))
            }
            CopilotAction::Skill(_s) => true,
            CopilotAction::Retreat(_r) => true,
            CopilotAction::SkillDaemon => true,
            CopilotAction::UseLess => false,
        }
    }
}

impl CopilotActionCondition {
    //todo: different condition may conflict
    pub(crate) fn check(&self, f: &Frame) -> bool {
        if let Some(k) = self.kills {
            if f.kill_count >= k.into() {
                return true;
            }
        }
        if let Some(c) = self.costs {
            if f.cost >= c.into() {
                return true;
            }
        }
        false
    }
}

impl Copilot {
    pub(crate) fn build_calculator<P: AsRef<Path>>(
        copilot_path: P,
        game_data_path: P,
    ) -> Result<Calculator> {
        let json = load_json_file(copilot_path)?;
        let copilot_data: CopilotData = serde_json::from_value(json)?;
        let loader = Loader::new(game_data_path)?;
        let mut calculator = loader.load_level_by_name(copilot_data.stage_name.clone())?;
        for o in copilot_data.operators.iter() {
            let skill_index = o.skill.unwrap_or(0);
            let (level, elite, skill_level) = match &o.requirement {
                Some(r) => (
                    r.level.unwrap_or(1),
                    r.elite.unwrap_or(0),
                    r.skill_level.unwrap_or(1),
                ),
                None => (1, 0, 1),
            };
            let op = Rc::new(RefCell::new(loader.load_operator(
                o.name.clone(),
                elite as usize,
                level as u32,
                skill_index as usize,
                skill_level as usize,
            )?));
            op.borrow_mut().self_weak = Rc::downgrade(&op); 
            let name = op.borrow().name.clone();
            calculator.frame_vec[0]
                .operator_undeploy
                .insert(name, op);
        }
        for g in copilot_data.groups.iter() {
            if let Some(o) = g.operators.choose(&mut thread_rng()) {
                let skill_index = o.skill.unwrap_or(0);
                let (level, elite, skill_level) = match &o.requirement {
                    Some(r) => (
                        r.level.unwrap_or(1),
                        r.elite.unwrap_or(0),
                        r.skill_level.unwrap_or(1),
                    ),
                    None => (1, 0, 1),
                };
                let op = loader.load_operator(
                    o.name.clone(),
                    elite as usize,
                    level as u32,
                    skill_index as usize,
                    skill_level as usize,
                )?;
                calculator.frame_vec[0]
                    .operator_undeploy
                    .insert(op.name.clone(), Rc::new(RefCell::new(op)));
            }
        }
        calculator.copilot = Some(Copilot {
            copilot_data,
        });
        Ok(calculator)
    }
    pub(crate) fn query(&self, f: &Frame) -> Vec<Event> {
        let mut v = Vec::new();
        for a in self.copilot_data.actions.iter() {
            if a.check(f) {
                if let Ok(e) = a.clone().try_into() {
                    v.push(e);
                    // we force only one action can be execute in one frame
                    break;
                }
            }
        }
        v
    }
}

mod test {
    

    
    
    #[test]
    fn test_copilot() {
        // let start = Instant::now();
        // let calculator =
        //     Copilot::build_calculator("./copilot.json", "./ArknightsGameData").unwrap();
        // let duration = start.elapsed();
    }
}
