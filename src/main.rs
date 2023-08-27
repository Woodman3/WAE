#![allow(non_snake_case)]
#![allow(unused)]
#![allow(dead_code)]
use log::{info, trace, warn};
// pub mod map;
// pub mod block;
pub mod calculator;
pub mod demo;
pub mod frame;
pub mod timeline;
pub mod unit;
pub mod utils;

fn main() {
    env_logger::init();
    demo::fun();
    // let c=utils::config::Config::new("/home/archer/workspace/BEC/config/").unwrap();
    // let c=utils::config::Config::new("C:/Users/Aureliano/workspace/BEC/config").unwrap();
    // let mut Ca = calculator::Calculator::new(&c).unwrap();
    // Ca.to_end();

    // println!("{:?}",Ca.enemy_initial)
}
