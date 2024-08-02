use serde_json::Value;
use serde::{Deserialize, Serialize};

use super::math::Grid;
use crate::timeline::doctor::{OperatorDeployEvent, OperatorRetreatEvent, OperatorSkillEvent, UnitRetreatEvent, UnitSkillEvent};
use crate::unit::scope::Toward;

#[derive(Debug, Serialize, Deserialize,Default)]
pub(super) struct Copilot{
    pub(super) stage_name: String,
    #[serde(rename = "opers")]
    pub(super) operators:Vec<CopilotOperator>,
    pub(super) groups:Vec<CopilotGroup>,
    pub(super) actions:Vec<CopilotAction>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub(super) struct CopilotOperator{
    pub(super) name:String,
    pub(super) skill:Option<u8>,
    pub(super) skill_usage:Option<u8>,
    pub(super) skill_times:Option<u8>,
    pub(super) requirement:Option<CopilotRequirement>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub(super) struct CopilotRequirement{
    pub(super) elite:Option<u8>,
    pub(super) level:Option<u8>,
    pub(super) skill_level:Option<u8>,
    pub(super) module:Option<u8>,
    pub(super) potential:Option<u8>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub(super) struct CopilotGroup{
    pub(super) name:String,
    #[serde(rename = "opers")]
    pub(super) operators:Vec<CopilotOperator>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub(super) enum CopilotAction{
    Deploy(CopilotActionDeploy),
    Skill(CopilotActionSkill),
    Retreat(CopilotActionRetreat),
    SkillDaemon,
    #[serde(other)]
    UseLess
}

#[derive(Debug, Serialize, Deserialize,Default)]
#[serde(default)]
pub(super) struct CopilotActionDeploy{
    pub(super) name:String,
    pub(super) location:Grid,
    pub(super) direction:Toward,
    pub(super) kills:Option<u8>,
    pub(super) costs:Option<u8>,
    pub(super) cost_changes:Option<u8>,
    pub(super) cooling:Option<i8>,
    pub(super) pre_delay:Option<u8>,
    pub(super) post_delay:Option<u8>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
#[serde(default)]
pub(super) struct CopilotActionSkill{
    pub(super) name:Option<String>,
    pub(super) skill_usage:Option<u8>,
    pub(super) skill_times:Option<u8>,
    pub(super) location:Option<Grid>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
#[serde(default)]
pub(super) struct CopilotActionRetreat{
    pub(super) name:Option<String>,
    pub(super) location:Option<Grid>,
}

impl Into<OperatorDeployEvent> for CopilotActionDeploy{
    fn into(self) -> OperatorDeployEvent {
        OperatorDeployEvent{
            operator_key:self.name.clone(),
            location:self.location,
            toward:self.direction,
        }
    }
}

impl TryInto<OperatorRetreatEvent> for CopilotActionRetreat{
    type Error=Box<dyn std::error::Error>;
    
    fn try_into(self) -> Result<OperatorRetreatEvent, Self::Error> {
        let operator_key = self.name.clone().ok_or("without operator name")?;
        Ok(OperatorRetreatEvent{
            operator_key,
        })
    }
}

impl TryInto<OperatorSkillEvent> for CopilotActionSkill{
    type Error=Box<dyn std::error::Error>;
    
    fn try_into(self) -> Result<OperatorSkillEvent, Self::Error> {
        let operator_key = self.name.clone().ok_or("without operator name")?;
        Ok(OperatorSkillEvent{
            operator_key,
        })
    }
}

impl TryInto<UnitSkillEvent> for CopilotActionSkill{
    type Error=Box<dyn std::error::Error>;
    
    fn try_into(self) -> Result<UnitSkillEvent, Self::Error> {
        let location = self.location.ok_or("without location")?;
        Ok(UnitSkillEvent{
            location,
        })
    }
}

impl TryInto<UnitRetreatEvent> for CopilotActionRetreat{
    type Error=Box<dyn std::error::Error>;
    
    fn try_into(self) -> Result<UnitRetreatEvent, Self::Error> {
        let location = self.location.ok_or( "without location")?;
        Ok(UnitRetreatEvent{
            location,
        })
    }
}

mod test{
    use super::*;
    use serde_json::json;
    use crate::utils::load_json_file;

    #[test]
    fn test_copilot(){
        let path = "copilot.json";
        let json = load_json_file(path).unwrap();
        let copilot: Copilot = serde_json::from_value(json).unwrap();
        println!("{:?}",copilot.actions);
    }
}