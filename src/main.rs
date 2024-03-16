#![allow(non_snake_case)]
#![allow(unused)]
#![allow(dead_code)]

use log::{info, trace, warn};
use crate::utils::visualizer::Visualizer;

//mod block;
mod calculator;
mod demo;
mod frame;
mod map;
mod timeline;
mod unit;
mod utils;

fn main() {
    use eframe::egui::vec2;
    env_logger::init();
    // let c=utils::config::Config::new("/home/archer/workspace/BEC/config/").unwrap();
    let c = utils::config::Config::new("C:/Users/Aureliano/workspace/WAE/config").unwrap();
    // demo::fun();
    let mut Ca = calculator::Calculator::new(&c).unwrap();
    let mut native_config = eframe::NativeOptions::default();
    native_config.viewport.inner_size = vec2(1000.0, 500.0).into();
    eframe::run_native("BEC", native_config, Box::new(|cc| {
        Box::new(Visualizer::new(cc,Ca))}));
    // Ca.to_end();
}

#[cfg(test)]
mod test{

}