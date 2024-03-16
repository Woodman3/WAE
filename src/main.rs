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
    native_config.initial_window_size = vec2(1000.0, 500.0).into();
    eframe::run_native("BEC", native_config, Box::new(|cc| {
        Box::new(Visualizer::new(cc,Ca))}));
    // Ca.to_end();
}

#[cfg(test)]
mod test{
    use std::{cell::RefCell, rc::Rc};
    use enum_dispatch::enum_dispatch;
    use serde::Serialize;
    use serde_json::to_string_pretty;

    #[enum_dispatch]
    trait Temptrait{}
    #[derive(Debug,Serialize)]
    struct A{v:i32}
    #[derive(Debug,Serialize)]
    struct B{v:f32}
    impl Temptrait for A{}
    impl Temptrait for B{}
    #[enum_dispatch(Temptrait)]
    #[derive(Debug,Serialize)]
    enum TempEnum{
        A,
        B,
    }    
    enum TempRef{
        A(Rc<RefCell<A>>),
        B(Rc<RefCell<B>>),
    }    

    #[test]
    fn t(){
        let a=A{v:1};
        let ra = Rc::new(RefCell::new(a));
        let rt:Temp = ra.into();
    }
}