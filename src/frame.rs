use crate::unit::enemy;
use crate::unit::enemy::Enemy;
use std::fmt;

#[derive(Debug,Clone)]
pub struct Frame{
    pub timestamp:u64,
    pub enemy_set:Vec<Enemy>
}

impl Frame {
    pub fn step(&mut self,t:f64){
        for mut e in self.enemy_set.iter_mut(){
            e.calculate_vector();
        }
        for mut e in self.enemy_set.iter_mut(){
            e.step(t)
        }
    }

}

impl fmt::Display for Frame{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"\
timestamp:{timestamp}
num of enemy:{enemy_len}
enemy info:"
               ,timestamp=self.timestamp,enemy_len=self.enemy_set.len())?;
        for e in self.enemy_set.iter(){
            writeln!(f,"{}",e)?;
        }
        write!(f,"")
    }
}