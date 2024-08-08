use crate::calculator::Calculator;
use crate::frame::Frame;
use crate::timeline::doctor::{OperatorDeployEvent, OperatorRetreatEvent, OperatorSkillEvent};
use crate::timeline::hostile::EnemyPlaceEvent;
use crate::utils::config::Config;
use crate::utils::error::ConfigParseError;
use doctor::{UnitRetreatEvent, UnitSkillEvent};
use serde::{Deserialize,Serialize};
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

#[derive(Debug,Deserialize,Serialize,Clone)]
pub(crate) enum Event{
    EnemyPlaceEvent(EnemyPlaceEvent),
    OperatorDeployEvent(OperatorDeployEvent),
    OperatorRetreatEvent(OperatorRetreatEvent),
    OperatorSkillEvent(OperatorSkillEvent),
    UnitRetreatEvent(UnitRetreatEvent),
    UnitSkillEvent(UnitSkillEvent),
}

impl Event {
    pub(crate) fn happen(&self, f: &mut Frame, c: &Calculator) {
        match self {
            Event::EnemyPlaceEvent(e) => e.happen(f, c),
            Event::OperatorDeployEvent(e) => e.happen(f, c),
            Event::OperatorRetreatEvent(e) => e.happen(f, c),
            Event::OperatorSkillEvent(e) => e.happen(f, c),
            Event::UnitRetreatEvent(e) => e.happen(f, c),
            Event::UnitSkillEvent(e) => e.happen(f, c),
        }
    }
}
#[derive(Debug)]
pub(super) struct EventWithTime {
    pub(super) time_stamp: u64,
    pub(super) event: Rc<Event>,
}

pub(super) fn read_timeline(c: &Config) -> Result<(VecDeque<EventWithTime>, u64)> {
    let mut time_line = VecDeque::<EventWithTime>::new();
    for v in c.doctor["timeline"].as_array().unwrap() {
        let time = from_value::<u64>(v[0].clone())?;
        let e = action_to_event(v)?;
        time_line.push_back(EventWithTime {
            time_stamp: time,
            event: Rc::clone(&e),
        });
    }
    let mut last_enemy_time: u64 = 0;
    for v in c.hostile["timeline"].as_array().unwrap() {
        let (time, op) = (
            from_value::<u64>(v[0].clone())?,
            from_value::<String>(v[1].clone())?,
        );
        let e: Rc<Event> = match op.as_str() {
            "Place" => Rc::new(Event::EnemyPlaceEvent(EnemyPlaceEvent::new(v)?)),
            _ => return Err(ConfigParseError("unknown op in timeline".into()).into()),
        };
        if time > last_enemy_time {
            last_enemy_time = time;
        }
        time_line.push_back(EventWithTime {
            time_stamp: time,
            event: Rc::clone(&e),
        });
    }
    Ok((time_line, last_enemy_time))
}

pub(crate) fn action_to_event(v: &Value) -> Result<Rc<Event>> {
    let op = from_value::<String>(v[1].clone())?;
    let e: Rc<Event> = match op.as_str() {
        "Deploy" => Rc::new(Event::OperatorDeployEvent(OperatorDeployEvent::new(v)?)),
        "Retreat" => Rc::new(Event::OperatorRetreatEvent(OperatorRetreatEvent {
            operator_key: v[2]
                .as_str()
                .ok_or(ConfigParseError(
                    "operator key can't translate to str in timeline".into(),
                ))?
                .into(),
        })),
        "Skill" => Rc::new(Event::OperatorSkillEvent(serde_json::from_value::<OperatorSkillEvent>(v[2].clone())?)),
        _ => return Err(ConfigParseError("unknown op in timeline".into()).into()),
    };
    Ok(e)
}
