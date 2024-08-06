// #![allow(non_snake_case)]
// #![allow(unused)]
// #![allow(dead_code)]

use log::{info, trace, warn};
use utils::visualizer::Visualizer;
use utils::data_loader::Loader;

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
    // let c = utils::config::Config::new("C:/Users/Aureliano/workspace/WAE/config").unwrap();
    // let mut Ca = calculator::Calculator::new(&c).unwrap();
    // let mut native_config = eframe::NativeOptions::default();
    // native_config.viewport.inner_size = vec2(1000.0, 500.0).into();
    // eframe::run_native("BEC", native_config, Box::new(|cc| {
    //     Box::new(Visualizer::new(cc,Ca))}));
    
    let l = Loader::new("ArknightsGameData").unwrap();
    let mut Ca = l.load_level("level_main_01-01".to_string()).unwrap();
    let mut native_config = eframe::NativeOptions::default();
    native_config.viewport.inner_size = vec2(1000.0, 500.0).into();
    eframe::run_native("BEC", native_config, Box::new(|cc| {
        Box::new(Visualizer::new(cc,Ca))}));


}

#[cfg(test)]
mod test{
    use super::*;
    use log::{info, trace, warn};
    use utils::data_loader::Loader;
    use crate::utils::visualizer::Visualizer;
    use eframe::egui::vec2;
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