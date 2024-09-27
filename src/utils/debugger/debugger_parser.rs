use std::{collections::HashMap, rc::Rc};

use regex::Regex;

use crate::{event::Event, frame::{timer::Timer, EnemyRef, Frame, OperatorRef}, map::Map, unit::bullet::Bullet, utils};

use utils::Result;

pub(super) enum Pointer{
    Frame(*const Frame),
    Timer(*const Timer),
    Enemy(EnemyRef),
    Enemies(*const Vec<EnemyRef>),
    Operators(*const HashMap<String,OperatorRef>),
    Operator(OperatorRef),
    Map(*const Map),
    BulletSet(*const Vec<Bullet>),
    Events(*const Vec<Event>),
    Usize(*const usize),
    U32(*const u32),
    F32(*const f32),
}

pub(super) unsafe fn parser(input: &str,f:& Frame)->Result<Pointer>{
    let re = Regex::new(r"^\s*(\w+)\s*(.*)").unwrap();
    if let Some(caps) = re.captures(input){
        let command = caps.get(1).unwrap().as_str();
        let object = caps.get(2).unwrap().as_str();
        match command{
            "p" => {
                let mut obj = Pointer::Frame(f as *const Frame);
                for field in object.split('.'){
                    if field.ends_with("]") {
                        let re = Regex::new(r"(\w+)\[(.*)\]").unwrap();
                        if let Some(caps) = re.captures(field){
                            let field = caps.get(1).unwrap().as_str();
                            let index = caps.get(2).unwrap().as_str();
                            match obj {
                                Pointer::Frame(f) => {
                                    match field{
                                        "enemy" => {
                                            let index: usize = index.parse().unwrap();
                                            obj = Pointer::Enemy(Rc::clone(&(*f).enemy_set[index]));
                                        },
                                        "operator" => {
                                            obj = Pointer::Operator(Rc::clone(&(*f).operator_deploy[index]));
                                        },
                                        "timer" => {
                                            obj = Pointer::Timer(&(*f).timer);
                                        },
                                        _ => {
                                            return Err("Invalid field".into())
                                        }
                                    }
                                },
                                _ => {
                                    return Err(format!("Invalid field: {}",field).into())
                                }
                            }
                        }else{
                            return Err(format!("can't parse field: {}",field).into())
                        }
                    }else{
                        return field_parser(&obj,field)
                    }
                }
                return Ok(obj)
            },
            _ => {
                return Err((format!("Invalid command: {}",command)).into())
            }
        }
    }
    return Err((format!("Invalid input: {}",input)).into())
}

unsafe fn field_parser(obj: &Pointer,field: &str)->Result<Pointer>{
    match obj{
        Pointer::Frame(f) => {
            match field{
                "enemy" => {
                    Ok(Pointer::Enemies(&(*(*f)).enemy_set))
                },
                "operator" => {
                    Ok(Pointer::Operators(&(*(*f)).operator_deploy))
                },
                "timer" => {
                    Ok(Pointer::Timer(&(*(*f)).timer))
                },
                _ => {
                    Err("Invalid field".into())
                }
            }
        },
        Pointer::Operator(o) => {
            match field{
                _ => {
                    Err("Invalid field".into())
                }
            }
        },
        _ => {
            Err("Invalid object".into())
        }
    }
}