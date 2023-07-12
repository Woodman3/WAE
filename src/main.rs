// pub mod map;
// pub mod block;
pub mod unit;
pub mod utils;
fn main() {
    // let m = map::read_for_json("/home/archer/workspace/BEC/config/").unwrap();
    // println!("{:?}",m);
    // let v=block::placeinfo::construct_place_info_from_json("/home/archer/workspace/BEC/config/").unwrap();     
    // println!("{:?}",v[..9]);
    // let r =untils::configloader::construct_place_info_from_json<unit::UnitInfo>("/home/archer/workspace/BEC/config/enemy.json")
    // println!("{}",std::any::type_name::<unit::UnitInfo>());
    // let t=utils::get_short_type_name::<unit::UnitInfo>();
    // println!("{t}");
    // let tn=std::any::type_name::<unit::UnitInfo>();
    //
    // if let Some(index) = tn.rfind(":")  {
    //     if let slice = &tn[index+1 ..]{
    //         println!("{slice}");
    //     }
    // }
    let v = utils::configloader::construct_info_from_json::<unit::UnitInfo>("/home/archer/workspace/BEC/config/enemy.json","",).unwrap();
    println!("{:?}",v);
    
}
