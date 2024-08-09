// #![allow(non_snake_case)]
#![allow(unused)]
#![allow(dead_code)]

use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use log::LevelFilter;
use utils::copilot::Copilot;
use utils::data_loader::Loader;
use utils::visualizer::{Debugger, DebugLogger};

//mod block;
mod calculator;
mod demo;
mod frame;
mod map;
mod route;
mod timeline;
mod unit;
mod utils;

fn main() {
    use eframe::egui::vec2;
    // env_logger::init();
    let (sender, receiver) = mpsc::channel();
    let logger = DebugLogger { sender };
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(LevelFilter::Info);


    let l = Loader::new("ArknightsGameData").unwrap();
    // let ca = l.load_level("main_01-01".to_string()).unwrap();
    let mut ca = Copilot::build_calculator("./copilot.json", "./ArknightsGameData").unwrap();
    let mut native_config = eframe::NativeOptions::default();
    native_config.viewport.inner_size = vec2(1000.0, 500.0).into();
    eframe::run_native(
        "BEC",
        native_config,
        Box::new(|cc| Box::new(Debugger{
            c: ca,
            run: false,
            log_receiver: Arc::new(Mutex::new(receiver)),
            log_messages: Arc::new(Mutex::new(Vec::new())),
        })),
    );
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_loader(){
    //     env_logger::init();
    //     let l = Loader::new("ArknightsGameData").unwrap();
    //     let mut Ca = l.load_level("level_main_01-07".to_string()).unwrap();
    //     let mut native_config = eframe::NativeOptions::default();
    //     native_config.viewport.inner_size = vec2(1000.0, 500.0).into();
    //     eframe::run_native("BEC", native_config, Box::new(|cc| {
    //         Box::new(Visualizer::new(cc,Ca))}));
    // }
}
