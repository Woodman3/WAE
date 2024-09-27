use std::collections::HashMap;

use regex::Regex;

use crate::{event::Event, frame::{timer::Timer, EnemyRef, Frame, OperatorRef}, map::Map, unit::bullet::Bullet, utils};

use utils::Result;

pub(super) enum Pointer<'a>{
    Frame(&'a Frame),
    Timer(&'a Timer),
    Enemies(&'a Vec<EnemyRef>),
    Operators(&'a HashMap<String,OperatorRef>),
    Map(&'a Map),
    BulletSet(&'a Vec<Bullet>),
    Events(&'a Vec<Event>),
    Usize(&'a usize),
    U32(&'a u32),
    F32(&'a f32),
}

pub(super) fn parser<'a>(input: &str,f:&'a Frame)->Result<Pointer<'a>>{
    let re = Regex::new(r"^\s*(\w+)\s*(.*)").unwrap();
    if let Some(caps) = re.captures(input){
        let command = caps.get(1).unwrap().as_str();
        let object = caps.get(2).unwrap().as_str();
        match command{
            "p" => {
                let mut obj = Pointer::Frame(&f);
                for field in object.split('.'){
                    if field.ends_with("]") {
                        let re = Regex::new(r"(\w+)\[(\d+)\]").unwrap();
                        if let Some(caps) = re.captures(field){
                            let field = caps.get(1).unwrap().as_str();
                            let mut index = caps.get(2).unwrap().as_str();
                            match obj {
                                Pointer::Frame(f) => {
                                    match field{
                                        "enemy" => {
                                            let index: usize = index.parse().unwrap();
                                            obj = Pointer::Enemies(&f.enemy_set);
                                        },
                                        "operator" => {
                                            obj = Pointer::Operators(&f.operator_deploy);
                                        },
                                        "timer" => {
                                            obj = Pointer::Timer(&f.timer);
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
