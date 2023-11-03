use std::collections::{HashMap,VecDeque};
use crate::calculator::Calculator;
use crate::frame::Frame;
use std::fmt::Debug;
use std::rc::Rc;
use crate::utils::config::Config;
use serde_json::from_value;
use crate::timeline::doctor::{OperatorDeployEvent, OperatorRetreatEvent, OperatorSkillEvent};
use crate::timeline::hostile::EnemyPlaceEvent;
use crate::utils::error::ConfigParseError;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod hostile;
pub mod doctor;

pub trait Event: Debug {
    fn happen(&self, f: &mut Frame, c: &Calculator);
}
#[derive(Debug)]
pub struct EventWithTime{
    pub time_stamp:u64,
    pub e:Rc<dyn Event>
}

pub fn read_doctor_timeline(c:&Config)->Result<(VecDeque<EventWithTime>,Vec<Rc<dyn Event>>)>{
    use crate::unit::scope::Toward;
    let mut time_line = VecDeque::<EventWithTime>::new();
    let mut event_set = Vec::<Rc::<dyn Event>>::new();
    for v in c.doctor["timeline"].as_array().unwrap() {
        let (time,op) = (from_value::<u64>(v[0].clone())?,from_value::<String>(v[1].clone())?);
        let e:Rc<dyn Event>= match op.as_str() {
            "Deploy" => {
                Rc::new(OperatorDeployEvent::new(v)?)
            }
            "Retreat" =>{
                Rc::new(OperatorRetreatEvent{operator_key:v[2].as_str().ok_or(ConfigParseError("operator key can't translate to str in timeline".into()))?.into()})
            }
            "Skill"=>{
                Rc::new(serde_json::from_value::<OperatorSkillEvent>(v[2].clone())?)
            }
            _ =>{ return Err(ConfigParseError("unknown op in timeline".into()).into())}
        };
        time_line.push_back(EventWithTime{time_stamp:time,e:Rc::clone(&e)});
        event_set.push(Rc::clone(&e));
    };
    for v in c.hostile["timeline"].as_array().unwrap() {
        let (time,op) = (from_value::<u64>(v[0].clone())?,from_value::<String>(v[1].clone())?);
        let e:Rc<dyn Event>= match op.as_str() {
            "Place" => {
                Rc::new(EnemyPlaceEvent::new(v)?)
            }
            _ =>{ return Err(ConfigParseError("unknown op in timeline".into()).into())}
        };
        time_line.push_back(EventWithTime{time_stamp:time,e:Rc::clone(&e)});
        event_set.push(Rc::clone(&e));
    };
    Ok((time_line,event_set))
}

