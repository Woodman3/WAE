use std::{collections::HashMap, rc::Rc};

use egui::Ui;
use log::error;
use regex::Regex;

use crate::{
    event::Event,
    frame::{timer::Timer, EnemyRef, Frame, OperatorRef},
    map::Map,
    unit::{bullet::Bullet, skill::skill_schedule::SkillSchedule, UnitInfo},
    utils,
};

use utils::Result;

#[derive(Default)]
pub(super) struct DebuggerParser {
    pub(super) paint_buffer: Vec<(String, Pointer)>,
    watch_buffer: Vec<(String, Pointer)>,
}

pub(super) enum Pointer {
    Frame(*const Frame),
    Timer(*const Timer),
    Enemy(EnemyRef),
    Enemies(*const Vec<EnemyRef>),
    Operators(*const HashMap<String, OperatorRef>),
    Operator(OperatorRef),
    Map(*const Map),
    BulletSet(*const Vec<Bullet>),
    Events(*const Vec<Event>),
    UnitInfo(*const UnitInfo),
    Skills(*const SkillSchedule),
    Usize(*const usize),
    U32(*const u32),
    U64(*const u64),
    F32(*const f32),
    I64(*const i64),
    /// 字符串应该不会被修改...应该吧，这里使用的是复制存值
    String(String),
    None,
}

impl DebuggerParser {
    pub(super) fn parse(&mut self, input: &str, f: &Frame) {
        match self.parse_command(input, f) {
            Ok(_) => {}
            Err(e) => {
                error!("parser error : {:?}", e);
            }
        }
    }

    fn parse_command(&mut self, input: &str, f: &Frame) -> Result<()> {
        let re = Regex::new(r"^\s*(\w+)\s*(.*)").unwrap();
        let caps = re
            .captures(input)
            .ok_or(format!("Invalid input: {}", input))?;
        let command = caps.get(1).unwrap().as_str();
        match command {
            "p" | "w" => unsafe {
                let object = caps.get(2).unwrap().as_str();
                let obj = self.parse_object(object, f)?;
                if command == "p" {
                    self.paint_buffer.push(obj);
                } else {
                    self.watch_buffer.push(obj);
                }
                Ok(())
            },
            "h" => {
                self.paint_buffer.push(("help".to_string(), Pointer::None));
                Ok(())
            }
            "l" => {
                let object = caps.get(2).unwrap().as_str();
                let info = match object {
                    "enemy" | "e" => {
                        let mut info = "enemy".to_string();
                        for e in f.enemy_set.iter() {
                            info.push_str(&format!("\n{:?}", e.borrow().name));
                        }
                        info
                    }
                    "operator" | "o" => {
                        let mut info = "operator".to_string();
                        for o in f.operator_deploy.values() {
                            info.push_str(&format!("\n{:?}", o.borrow().name));
                        }
                        info
                    }
                    _ => return Err(format!("Invalid object: {}", object).into()),
                };
                self.paint_buffer.push((info, Pointer::None));
                Ok(())
            }
            _ => Err(format!("Invalid command: {}", command).into()),
        }
    }
    unsafe fn parse_object(&mut self, object: &str, f: &Frame) -> Result<(String, Pointer)> {
        let mut obj = Pointer::Frame(f as *const Frame);
        for field in object.split('.') {
            if field.ends_with("]") {
                let re = Regex::new(r"(\w+)\[(.*)\]").unwrap();
                let caps = re
                    .captures(field)
                    .ok_or(format!("can't parse field: {}", field))?;
                let field = caps.get(1).unwrap().as_str();
                let index = caps.get(2).unwrap().as_str();
                obj = match obj {
                    Pointer::Frame(f) => match field {
                        "enemy" | "e" => {
                            let index: usize = index.parse().unwrap();
                            Pointer::Enemy(Rc::clone(&(*f).enemy_set[index]))
                        }
                        "operator" | "o" => {
                            Pointer::Operator(Rc::clone(&(*f).operator_deploy[index]))
                        }
                        _ => return Err("Invalid field".into()),
                    },
                    _ => return Err(format!("Invalid field: {}", field).into()),
                }
            } else {
                obj = parse_field(&obj, field)?;
            }
        }
        Ok((object.to_string(), obj))
    }

    pub(super) unsafe fn show_pointer(&self, ui: &mut Ui) {
        for (o, p) in self.paint_buffer.iter().chain(self.watch_buffer.iter()) {
            match p {
                Pointer::Frame(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::Enemies(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::Operators(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::Map(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::BulletSet(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::Events(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::Usize(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::U32(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::F32(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::Timer(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::Enemy(obj) => {
                    ui.label(format!("{o}:\n {:?}", obj));
                }
                Pointer::Operator(obj) => {
                    ui.label(format!("{o}:\n {:?}", obj));
                }
                Pointer::U64(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::I64(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::UnitInfo(obj) => {
                    ui.label(format!("{o}:\n {:?}", **obj));
                }
                Pointer::Skills(obj) => {
                    ui.label(format!("{o}:\n {}", **obj));
                }
                Pointer::String(obj) => {
                    ui.label(format!("{o}:\n {:?}", obj));
                }
                Pointer::None => {
                    ui.label(format!("{o}"));
                }
            }
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

unsafe fn parse_field(obj: &Pointer, field: &str) -> Result<Pointer> {
    match obj {
        Pointer::Frame(f) => {
            match_field!(field,
                "enemy" => Pointer::Enemies(&(**f).enemy_set),
                "operator" => Pointer::Operators(&(**f).operator_deploy),
                "timer" => Pointer::Timer(&(**f).timer)
            )
        }
        Pointer::Timer(t) => {
            match_field!(field,
                "timestamp" => Pointer::U64(&(**t).timestamp)
            )
        }
        Pointer::Enemy(e) => {
            match_field!(field,
                "name" => Pointer::String(e.borrow().name.clone()),
                "stage" => Pointer::UnitInfo(&e.borrow().stage)
            )
        }
        Pointer::Operator(o) => {
            match_field!(field,
                "name" => Pointer::String(o.borrow().name.clone()),
                "stage" => Pointer::UnitInfo(&o.borrow().stage),
                "skills" => Pointer::Skills(&o.borrow().skills)
            )
        }
        _ => Err("Invalid object".into()),
    }
}
