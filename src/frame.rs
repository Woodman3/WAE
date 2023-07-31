use crate::unit::enemy;
use crate::unit::enemy::Enemy;

#[derive(Debug,Clone)]
pub struct Frame{
    pub enemy_position:Vec<(f64,f64)>,
    pub enemy_set:Vec<Enemy>
}

impl Frame {
    pub fn step(&mut self,t:f64){
        for mut e in self.enemy_set.iter_mut(){
            e.step(t)
        }
    }
}
