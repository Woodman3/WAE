use std::collections::HashMap;
use crate::unit::enemy::Enemy;
use crate::unit::code;
use crate::unit::operator::Operator;
use log::info;
use std::fmt;
use crate::calculator::Calculator;

#[derive(Debug, Clone)]
pub struct Frame {
    pub timestamp: u64,
    pub enemy_set: Vec<Enemy>,
    pub operator_deploy:HashMap<String,Operator>,
    pub operator_undeploy:HashMap<String,Operator>,
}

impl Frame {
    pub fn step(&mut self,c:&mut Calculator, t: f64) {
        // for mut i in 0..self.enemy_set.len() {
        //     self.enemy_set[i].calculate_vector();
        //     self.enemy_set[i].step(t);
        //     if (self.enemy_set[i].die_code == marco::INTO_END) {
        //         info!("An enemy has enter to blue point");
        //     }
        // }
        self.enemy_set.iter_mut().for_each(|e| {
            e.calculate_vector();
            e.step(t);
            if (e.die_code == code::INTO_END) {
                info!("An enemy has enter to blue point");
            }
        }
        );
        self.enemy_set.retain(|e| e.die_code!=code::INTO_END);
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "\
timestamp:{timestamp}
num of enemy:{enemy_len}
enemy info:",
            timestamp = self.timestamp,
            enemy_len = self.enemy_set.len()
        )?;
        for e in self.enemy_set.iter() {
            writeln!(f, "{}", e)?;
        }
        write!(f, "")
    }
}
