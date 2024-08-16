// #![allow(non_snake_case)]
#![allow(unused)]
#![allow(dead_code)]

use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use log::LevelFilter;
use utils::copilot::Copilot;
use utils::data_loader::Loader;
use utils::debugger::{Debugger, DebugLogger};

//mod block;
mod calculator;
mod demo;
mod frame;
mod map;
mod route;
mod timeline;
mod unit;
mod utils;
mod spawner;

fn main() {
    use eframe::egui::vec2;
    // env_logger::init();
    let (sender, receiver) = mpsc::channel();
    let logger = DebugLogger { sender };
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(LevelFilter::Debug);


    // let l = Loader::new("ArknightsGameData").unwrap();
    // let ca = l.load_level("main_01-01".to_string()).unwrap();
    let mut ca = Copilot::build_calculator("./copilot.json", "./ArknightsGameData").unwrap();
    let mut native_config = eframe::NativeOptions::default();
    native_config.viewport.inner_size = vec2(1300.0, 500.0).into();
    eframe::run_native(
        "BEC",
        native_config,
        Box::new(|cc| Ok(Box::new(Debugger{
            c: ca,
            run: false,
            log_receiver: Arc::new(Mutex::new(receiver)),
            log_messages: Arc::new(Mutex::new(Vec::new())),
        }))),
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_loader(){
        env_logger::init();
        let mut ca = Copilot::build_calculator("./copilot.json", "./ArknightsGameData").unwrap();
        let mut f = ca.frame_vec.pop().unwrap();
        let r = f.enemy_set[0].borrow().route.clone();
        let dis = f.map.spfa((0,10).into(),(2,0).into()); 
        println!("{:?}",dis);
    }
}
