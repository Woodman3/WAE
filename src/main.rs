#![allow(non_snake_case)]
#![allow(unused)]
#![allow(dead_code)]
use log::{info, trace, warn};
//mod block;
mod calculator;
mod demo;
mod frame;
mod timeline;
mod unit;
mod utils;
mod map;

fn main() {
    env_logger::init();
    // demo::fun();
    // let c=utils::config::Config::new("/home/archer/workspace/BEC/config/").unwrap();
    let c=utils::config::Config::new("C:/Users/Aureliano/workspace/BEC/config").unwrap();
    let mut Ca = calculator::Calculator::new(&c).unwrap();
    // println!("{:?}",Ca.frame_vec[0].operator_undeploy);
    Ca.to_end();
}
