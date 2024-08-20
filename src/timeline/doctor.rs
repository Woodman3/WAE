use super::Event;
use crate::calculator::Calculator;
use crate::frame::{Frame, OperatorRef};
use crate::unit::scope::Toward;
use crate::utils::error::ConfigParseError;
use crate::utils::math::Grid;
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::rc::Rc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug,Deserialize,Serialize,Clone,Default)]
pub(crate) struct OperatorDeployEvent {
    pub(crate) operator_key: String,
    pub(crate) location: Grid,
    pub(crate) toward: Toward,
}
#[derive(Debug,Deserialize,Serialize,Clone,Default)]
pub(crate) struct OperatorRetreatEvent {
    pub(crate) operator_key: String,
}

#[derive(Debug,Deserialize,Serialize,Clone,Default)]
pub(crate) struct OperatorSkillEvent {
    pub(crate) operator_key: String,
}

#[derive(Debug,Deserialize,Serialize,Clone,Default)]
pub(crate) struct UnitRetreatEvent {
    pub(crate) location: Grid,
}

#[derive(Debug,Deserialize,Serialize,Clone,Default)]
pub(crate) struct UnitSkillEvent {
    pub(crate) location: Grid,
}

impl OperatorDeployEvent {
    pub(super) fn new(v: &Value) -> Result<OperatorDeployEvent> {
        use serde_json::from_value;
        let location = (
            from_value::<i64>(v[3].clone())?,
            from_value::<i64>(v[4].clone())?,
        )
            .into();
        let t = from_value::<String>(v[5].clone())?;
        let toward = match t.as_str() {
            "South" => Toward::South,
            "West" => Toward::West,
            "North" => Toward::North,
            "East" => Toward::East,
            _ => Toward::East,
        };
        Ok(OperatorDeployEvent {
            operator_key: v[2]
                .as_str()
                .ok_or(ConfigParseError(
                    "operator key can't translate to str in timeline".into(),
                ))?
                .into(),
            location,
            toward,
        })
    }
}
impl OperatorDeployEvent {
    pub(super) fn happen(&self, f: &mut Frame, _c: &Calculator) {
        let or = f.operator_undeploy.remove(&self.operator_key).unwrap();
        let mut o = or.borrow_mut();
        o.location = self.location;
        let loc: (i32, i32) = (
            self.location.row.try_into().unwrap(),
            self.location.col.try_into().unwrap(),
        );
        o.search_scope = o.attack_scope.clone();
        o.search_scope.apply_toward(&self.toward);
        o.search_scope.apply_loc(loc, f.map.width, f.map.height);
        o.generate_default_attack_skill();
        f.operator_deploy
            .insert(self.operator_key.clone(), Rc::clone(&or));
        f.map.operator[self.location.row as usize][self.location.col as usize] =
            Some(self.operator_key.clone());
        if f.cost>=o.stage.cost as f32{
            f.cost-=o.stage.cost as f32;
        }else{
            error!("cost not enough");
        }
    }
}

impl OperatorRetreatEvent {
    pub(super) fn happen(&self, f: &mut Frame, _c: &Calculator) {
        let or: OperatorRef = f.operator_deploy.remove(&self.operator_key).unwrap();
        let o = or.borrow_mut();
        f.map.operator[o.location.row.clone() as usize][o.location.col.clone() as usize] = None;
        f.operator_undeploy
            .insert(self.operator_key.clone(), Rc::clone(&or));
    }
}

impl OperatorSkillEvent {
    pub(super) fn happen(&self, f: &mut Frame, _c: &Calculator) {
        if let Some(_o) = f.operator_deploy.get(self.operator_key.as_str()) {
            todo!()
        }
    }
}

impl UnitRetreatEvent {
    pub(super) fn happen(&self, f: &mut Frame, _c: &Calculator) {
        let remove_key =
            f.map.operator[self.location.row as usize][self.location.col as usize].clone();
        if let Some(key) = remove_key {
            let or: OperatorRef = f.operator_deploy.remove(&key).unwrap();
            f.operator_undeploy.insert(key.clone(), Rc::clone(&or));
            f.map.operator[self.location.row as usize][self.location.col as usize] = None;
        }
    }
}

impl UnitSkillEvent {
    pub(super) fn happen(&self, f: &mut Frame, c: &Calculator) {
        todo!()
    }
}
