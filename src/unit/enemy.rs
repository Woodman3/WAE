use serde_json::Value;
use std::rc::Rc;
use std::fmt;
use crate::utils::math;
#[derive(Debug,Clone)]
pub struct Enemy{
    info:super::UnitInfo,
    pub location:(f64,f64),/// -1 mean haven't place
    move_speed:u32,
    component_x:f64,///compoment of vector
    component_y:f64,
    route_stage:usize,
    // route:Option<&'a Vec<Vec<(f64,f64)>>>
    pub route:Option<Rc<Vec<(f64,f64)>>>
}

impl Enemy {
    /// t is 1/fps it mean time interval
    pub fn step(&mut self,t:f64){
        self.location.0+=self.move_speed as f64 *self.component_x *t;
        self.location.1+=self.move_speed as f64 *self.component_y *t;
    }
    pub fn new(v:&Value)->Enemy{
        Enemy{
            info:serde_json::from_value::<super::UnitInfo>(v["UnitInfo"].clone()).unwrap(),
            location:(-1 as f64,-1 as f64),
            move_speed:serde_json::from_value::<u32>(v["move_speed"].clone()).unwrap(),
            route_stage:1,
            component_x:0 as f64,
            component_y:0 as f64,
            route:None
        }
    }
    pub fn calculate_vector(&mut self){
        if let Some(route) = &self.route{
            if let Some((end_x,end_y)) = route.get(self.route_stage){
                let (delta_x,delta_y)=(self.location.0-end_x,self.location.1-end_y);
                let theta = delta_y.atan2(delta_x);
                self.component_x=theta.cos();
                self.component_y=theta.sin();
            }
        }
    }

}

impl fmt::Display for Enemy{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"\
location:{},{}
component_x:{} component_y:{} ",self.location.0,self.location.1,
        self.component_x,self.component_y)
    }

}