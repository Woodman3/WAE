use crate::calculator::Calculator;
use crate::frame::{Frame, OperatorRef};
use crate::event::doctor::{OperatorDeployEvent, OperatorRetreatEvent, OperatorSkillEvent};
use crate::event::hostile::EnemyPlaceEvent;
use crate::unit::operator::OperatorShared;
use crate::utils::config::Config;
use crate::utils::error::ConfigParseError;
use doctor::{UnitRetreatEvent, UnitSkillEvent};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::Rc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(crate) mod doctor;
pub(crate) mod hostile;

// pub(crate) trait Event: Debug {
//     fn happen(&self, f: &mut Frame, c: &Calculator);
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) enum Event {
    EnemyPlaceEvent(EnemyPlaceEvent),
    EnemyDieEvent(usize),
    EnemyEnterEvent(usize),
    OperatorDeployEvent(OperatorDeployEvent),
    OperatorRetreatEvent(String),
    OperatorSkillEvent(OperatorSkillEvent),
    UnitRetreatEvent(UnitRetreatEvent),
    UnitSkillEvent(UnitSkillEvent),
}

impl Event {
    pub(crate) fn happen(&self, f: &mut Frame, c: &mut Calculator) {
        match self {
            Event::EnemyPlaceEvent(e) => e.happen(f, c),
            Event::OperatorDeployEvent(e) => e.happen(f, c),
            Event::OperatorRetreatEvent(operator_key) => {
                let or: OperatorRef = f.operator_deploy.remove(operator_key).unwrap();
                let o = or.borrow_mut();
                f.map.operator[o.location.row.clone() as usize][o.location.col.clone() as usize] =
                    OperatorShared::new();
                f.operator_undeploy
                    .insert(operator_key.clone(), Rc::clone(&or));
                info!("an operator retreat");
            },
            Event::OperatorSkillEvent(e) => e.happen(f, c),
            Event::UnitRetreatEvent(e) => e.happen(f, c),
            Event::UnitSkillEvent(e) => e.happen(f, c),
            Event::EnemyDieEvent(id) => {
                f.enemy_set.retain(|e| e.borrow().id != *id);
                f.kill_count += 1;
                info!("an enemy die");
            },
            Event::EnemyEnterEvent(id) =>{
                f.enemy_set.retain(|e| e.borrow().id != *id);
                info!("an enemy enter end");
            },
        }
    }
}

pub(crate) fn action_to_event(v: &Value) -> Result<Rc<Event>> {
    let op = from_value::<String>(v[1].clone())?;
    let e: Rc<Event> = match op.as_str() {
        "Deploy" => Rc::new(Event::OperatorDeployEvent(OperatorDeployEvent::new(v)?)),
        // "Retreat" => Rc::new(Event::OperatorRetreatEvent(OperatorRetreatEvent {
        //     operator_key: v[2]
        //         .as_str()
        //         .ok_or(ConfigParseError(
        //             "operator key can't translate to str in timeline".into(),
        //         ))?
        //         .into(),
        // })),
        "Skill" => Rc::new(Event::OperatorSkillEvent(serde_json::from_value::<
            OperatorSkillEvent,
        >(v[2].clone())?)),
        _ => return Err(ConfigParseError("unknown op in timeline".into()).into()),
    };
    Ok(e)
}
