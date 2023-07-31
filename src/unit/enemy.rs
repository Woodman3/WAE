#[derive(Debug,Clone)]
pub struct Enemy{
    info:super::UnitInfo,
    location:(f64,f64),
    speed:u32,
    compoment_x:f64,
    compoment_y:f64
}

impl Enemy {
    /// t is 1/fps it mean time interval
    pub fn step(&mut self,t:f64){
        let (mut x,mut y)=self.location;
        x+=self.speed as f64 *self.compoment_x*t;
        y+=self.speed as f64 *self.compoment_y*t;
    }
}