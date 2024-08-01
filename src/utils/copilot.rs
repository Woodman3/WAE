use serde_json::Value;
use serde::{Deserialize, Serialize};

use super::math::Grid;
use crate::timeline::doctor::{OperatorDeployEvent,OperatorRetreatEvent,OperatorSkillEvent};
use crate::unit::scope::Toward;

#[derive(Debug, Serialize, Deserialize,Default)]
pub(super) struct Copilot{
    pub(super) stage_name: String,
    #[serde(rename = "opers")]
    pub(super) operators:Vec<Value>,
    pub(super) groups:Vec<Value>,
    pub(super) actions:Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub(super) struct CopilotOperator{
    pub(super) name:String,
    pub(super) skill:u8,
    pub(super) skill_usage:u8,
    pub(super) skill_times:u8,
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
    // #[serde(alias = "SpeedUp")]
    // Useless,
    // BulletTime,
    // SkillUsage,
    SkillDaemon,
}
// struct CopilotAction{
//     #[serde(rename = "type")]
//     action_type:Option<String>,
//     kills:Option<u8>,
//     costs:Option<u8>,
//     cost_changes:Option<u8>,
//     cooling:Option<i8>,
//     name:Option<String>,
//     location:Option<Grid>,
//     direction:Option<String>,
//     skill_usage:Option<u8>,
//     skill_times:Option<u8>,
//     pre_delay:Option<u8>,
//     post_delay:Option<u8>,
// }

#[derive(Debug, Serialize, Deserialize,Default)]
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
pub(super) struct CopilotActionSkill{
    pub(super) name:String,
    pub(super) skill_usage:Option<u8>,
    pub(super) skill_times:Option<u8>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub(super) struct CopilotActionRetreat{
    pub(super) name:String,
}

impl Into<OperatorDeployEvent> for CopilotActionDeploy{
    fn into(self)->OperatorDeployEvent{
        OperatorDeployEvent{
            operator_key:self.name.clone(),
            location:self.location.clone(),
            toward:self.direction,
        }
    }
}

impl Into<OperatorRetreatEvent> for CopilotActionRetreat{
    fn into(self)->OperatorRetreatEvent{
        OperatorRetreatEvent{
            operator_key:self.name.clone(),
        }
    }
}

impl Into<OperatorSkillEvent> for CopilotActionSkill{
    fn into(self)->OperatorSkillEvent{
        OperatorSkillEvent{
            operator_key:self.name.clone(),
        }
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