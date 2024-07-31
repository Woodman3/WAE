use serde_json::Value;
use serde::{Deserialize, Serialize};

use super::math::Grid;

#[derive(Debug, Serialize, Deserialize,Default)]
struct Copilot{
    stage_name: String,
    #[serde(rename = "opers")]
    operators:Vec<Value>,
    groups:Vec<Value>,
    actions:Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
struct CopilotOperator{
    name:String,
    skill:u8,
    skill_usage:u8,
    skill_times:u8,
    requirement:Option<CopilotRequirement>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
struct CopilotRequirement{
    elite:Option<u8>,
    level:Option<u8>,
    skill_level:Option<u8>,
    module:Option<u8>,
    potential:Option<u8>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
struct CopilotGroup{
    name:String,
    #[serde(rename = "opers")]
    operators:Vec<CopilotOperator>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
struct CopilotAction{
    #[serde(rename = "type")]
    action_type:Option<String>,
    kills:Option<u8>,
    costs:Option<u8>,
    cost_changes:Option<u8>,
    cooling:Option<i8>,
    name:Option<String>,
    location:Option<Grid>,
    direction:Option<String>,
    skill_usage:Option<u8>,
    skill_times:Option<u8>,
    pre_delay:Option<u8>,
    post_delay:Option<u8>,
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
        println!("{:?}",copilot);
    }
}