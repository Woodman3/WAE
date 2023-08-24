use crate::unit::enemy::Enemy;
use crate::unit::marco;
use std::fmt;
use log::info;

#[derive(Debug,Clone)]
pub struct Frame{
    pub timestamp:u64,
    pub enemy_set:Vec<Enemy>
}

impl Frame {
    pub fn step(&mut self,t:f64){
        for mut i in 0..self.enemy_set.len(){
            self.enemy_set[i].calculate_vector();
            self.enemy_set[i].step(t);
            if(self.enemy_set[i].die_code==marco::INTO_END){
                self.enemy_set.remove(i);
                info!("An enemy has enter to blue point");

            }
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