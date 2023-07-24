// pub mod map;
// pub mod block;
pub mod unit;
pub mod utils;
pub mod timeline;
// pub mod demo;
pub mod frame;
pub mod calculator;
fn main() {
    // let m = map::read_for_json("/home/archer/workspace/BEC/config/").unwrap();
    // println!("{:?}",m);
    // let v=block::placeinfo::construct_place_info_from_json("/home/archer/workspace/BEC/config/").unwrap();     
    // let v = utils::configloader::construct_info_from_json::<unit::UnitInfo>("/home/archer/workspace/BEC/config/enemy.json","",).unwrap();
    // println!("{:?}",v);
    // demo::fun();
    let c=utils::config::Config::new("/home/archer/workspace/BEC/config/").unwrap();
    let mut Ca = calculator::Calculator::new(&c).unwrap();
    Ca.to_end();
    println!("{:?}",Ca.route);
}
