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
    U64(*const u64),
    F32(*const f32),
}

pub(super) unsafe fn parser(input: &str,f:& Frame)->Result<Pointer>{
    let re = Regex::new(r"^\s*(\w+)\s*(.*)").unwrap();
    let caps = re.captures(input).ok_or(format!("Invalid input: {}",input))?;
    let command = caps.get(1).unwrap().as_str();
    let object = caps.get(2).unwrap().as_str();
    match command{
        "p" => {
            let mut obj = Pointer::Frame(f as *const Frame);
            for field in object.split('.'){
                if field.ends_with("]") {
                    let re = Regex::new(r"(\w+)\[(.*)\]").unwrap();
                    let caps = re.captures(field).ok_or(format!("can't parse field: {}",field))?;
                    let field = caps.get(1).unwrap().as_str();
                    let index = caps.get(2).unwrap().as_str();
                    obj = match obj {
                        Pointer::Frame(f) => {
                            match field{
                                "enemy" => {
                                    let index: usize = index.parse().unwrap();
                                    Pointer::Enemy(Rc::clone(&(*f).enemy_set[index]))
                                },
                                "operator" => {
                                    Pointer::Operator(Rc::clone(&(*f).operator_deploy[index]))
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
                    obj= field_parser(&obj,field)?;
                }
            }
            Ok(obj)
        },
        _ => {
            Err((format!("Invalid command: {}",command)).into())
        }
    }
}

macro_rules! match_field {
    ($field:expr, $($pattern:pat => $result:expr),*) => {
        match $field {
            $(
                $pattern => Ok($result),
            )*
            _ => Err("Invalid field".into()),
        }
    };
}

unsafe fn field_parser(obj: &Pointer, field: &str) -> Result<Pointer> {
    match obj {
        Pointer::Frame(f) => {
            match_field!(field,
                "enemy" => Pointer::Enemies(&(**f).enemy_set),
                "operator" => Pointer::Operators(&(**f).operator_deploy),
                "timer" => Pointer::Timer(&(**f).timer)
            )
        },
        Pointer::Timer(t) => {
            match_field!(field,
                "timestamp" => Pointer::U64(&(**t).timestamp)
            )
        },
        _ => Err("Invalid object".into()),
    }
}