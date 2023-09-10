use serde::Deserialize;
use serde_json::Value;
use super::Event;
use crate::calculator::Calculator;
use crate::frame::Frame;
use crate::unit::operator::Operator;
use crate::unit::scope::{Scope, Toward};
use crate::utils::error::ConfigParseError;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug,Clone)]
pub struct OperatorDeployEvent{
    operator_key:String,
    location:(u32,u32),
    toward:Toward,
}
#[derive(Debug,Deserialize)]
pub struct OperatorRetreatEvent{
    pub operator_key:String,
}

impl OperatorDeployEvent{
    pub fn new(v:&Value)->Result<OperatorDeployEvent>{
        use serde_json::from_value;
        let location:(u32,u32)=(from_value::<u32>(v[3].clone())?,from_value::<u32>(v[4].clone())?);
        let t =from_value::<String>(v[5].clone())?;
        let toward = match t.as_str() {
            "South" => {Toward::South}
            "West" =>{Toward::West}
            "North" => {Toward::North}
            "East" => {Toward::East}
            _ => {Toward::East}
        };
        Ok(OperatorDeployEvent{
            operator_key:v[2].as_str().ok_or(ConfigParseError("operator key can't translate to str in timeline".into()))?.into(),
            location,
            toward
        })

    }
}
impl Event for OperatorDeployEvent {
    fn happen(&self, f: &mut Frame, c: &Calculator) {
        let mut o :Operator=f.operator_undeploy.remove(&self.operator_key).unwrap();
        o.location=self.location;
        let width=f.map.width;
        let heigh = f.map.height;

        f.operator_deploy.insert(self.operator_key.clone(),o);
    }
}

impl Event for OperatorRetreatEvent{
    fn happen(&self, f: &mut Frame, c: &Calculator) {
        let mut o :Operator=f.operator_deploy.remove(&self.operator_key).unwrap();
        f.operator_undeploy.insert(self.operator_key.clone(),o);
    }
}
