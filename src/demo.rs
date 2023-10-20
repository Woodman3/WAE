use serde::Deserialize;
use crate::utils::config::Config;
use crate::skill::Skill;
pub  fn fun(c:&Config){
    let skill=&c.skill;
    let t:Skill=serde_json::from_value(skill["Operator"]["Skadi"]["skill1"].clone()).unwrap();
    println!("{:?}",t);
}
