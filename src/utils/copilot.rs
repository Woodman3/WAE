use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use super::data_loader::Loader;
use super::load_json_file;
use super::math::Grid;
use crate::calculator::Calculator;
use crate::timeline::doctor::{
    OperatorDeployEvent, OperatorRetreatEvent, OperatorSkillEvent, UnitRetreatEvent, UnitSkillEvent,
};
use crate::unit::scope::Toward;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug)]
struct Copilot {
    copilot_data: CopilotData,
    game_data: Loader,
    calculator: Calculator,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(super) enum CopilotAction {
    Deploy(CopilotActionDeploy),
    Skill(CopilotActionSkill),
    Retreat(CopilotActionRetreat),
    SkillDaemon,
    #[serde(other)]
    UseLess,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub(super) struct CopilotActionDeploy {
    pub(super) name: String,
    pub(super) location: Grid,
    pub(super) direction: Toward,
    /// after how many kill, cost, or cost change we deploy this operator
    pub(super) kills: Option<u8>,
    pub(super) costs: Option<u8>,
    pub(super) cost_changes: Option<u8>,
    pub(super) cooling: Option<i8>,
    pub(super) pre_delay: Option<u8>,
    pub(super) post_delay: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub(super) struct CopilotActionSkill {
    pub(super) name: Option<String>,
    pub(super) skill_usage: Option<u8>,
    pub(super) skill_times: Option<u8>,
    pub(super) location: Option<Grid>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

impl Copilot {
    pub(crate) fn new<P: AsRef<Path>>(copilot_path: P, game_data_path: P) -> Result<Self> {
        let json = load_json_file(copilot_path)?;
        let copilot_data: CopilotData = serde_json::from_value(json)?;
        let loader = Loader::new(game_data_path)?;
        let mut calculator = loader.load_level(copilot_data.stage_name.clone())?;
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
        Ok(Copilot {
            copilot_data,
            game_data: loader,
            calculator,
        })
    }
    pub(crate) fn run(&mut self){
        let c = &mut self.calculator;
        while c.step() {
            if let Some(f) = c.frame_vec.last() {
                for a in self.copilot_data.actions.iter(){
                    match a {
                        CopilotAction::Deploy(d) => {
                            todo!()
                        }
                        CopilotAction::Skill(s) => {
                            todo!()
                        }
                        CopilotAction::Retreat(r) => {
                            todo!()
                        }
                        CopilotAction::SkillDaemon => {
                            todo!()
                        }
                        CopilotAction::UseLess => {}
                    }
                }
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_copilot() {
        let copilot = Copilot::new("./copilot.json", "./ArknightsGameData").unwrap();
        println!("{:?}", copilot.calculator);
    }
}
