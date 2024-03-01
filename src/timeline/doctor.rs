use std::rc::Rc;
use serde::Deserialize;
use serde_json::Value;
use super::Event;
use crate::calculator::Calculator;
use crate::frame::{Frame, OperatorRef};
use crate::unit::scope::{Toward};
use crate::utils::error::ConfigParseError;
use crate::utils::math::Grid;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug,Clone,Default)]
pub struct OperatorDeployEvent{
    operator_key:String,
    location:Grid,
    toward:Toward,
}
#[derive(Debug,Deserialize,Default)]
pub struct OperatorRetreatEvent{
    pub operator_key:String,
}

#[derive(Debug,Deserialize,Default)]
pub struct OperatorSkillEvent{
    pub operator_key:String,
}

impl OperatorDeployEvent{
    pub fn new(v:&Value)->Result<OperatorDeployEvent>{
        use serde_json::from_value;
        let location=(from_value::<i64>(v[3].clone())?,from_value::<i64>(v[4].clone())?).into();
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
    fn happen(&self, f: &mut Frame, _c: &Calculator) {
        let or=f.operator_undeploy.remove(&self.operator_key).unwrap();
        let mut o=or.borrow_mut();
        o.location=self.location;
        let loc:(i32,i32)=(self.location.row.try_into().unwrap(),self.location.col.try_into().unwrap());
        o.search_scope=o.attack_scope.clone();
        o.search_scope.apply_toward(&self.toward);
        o.search_scope.apply_loc(loc,f.map.width,f.map.height);
        o.generate_default_attack_skill();
        f.operator_deploy.insert(self.operator_key.clone(),Rc::clone(&or));
        f.map.operator[self.location.row as usize][self.location.col as usize]=Some(self.operator_key.clone());
    }
}

impl Event for OperatorRetreatEvent{
    fn happen(&self, f: &mut Frame, _c: &Calculator) {
        let or:OperatorRef=f.operator_deploy.remove(&self.operator_key).unwrap();
        let o=or.borrow_mut();
        f.map.operator[o.location.row.clone() as usize][o.location.col.clone() as usize]=None;
        f.operator_undeploy.insert(self.operator_key.clone(),Rc::clone(&or));
    }
}

impl Event for OperatorSkillEvent{
    fn happen(&self, f: &mut Frame, _c: &Calculator) {
        if let Some(_o)=f.operator_deploy.get(self.operator_key.as_str()){

        }
    }
}