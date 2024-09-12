use std::{ops::{AddAssign},fmt};

use serde::{Deserialize, Serialize};

use crate::calculator::PERIOD;



#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub(crate) struct Timer{
    pub(crate) timestamp: u64,
    pub(crate) global: f64,
    pub(crate) wave: f64,
    pub(crate) subwave: f64,
}

impl Timer{
    pub(crate) fn step(&mut self){
        self.timestamp += 1;
        *self+=PERIOD;
    }
}

impl AddAssign<f64> for Timer{
    fn add_assign(&mut self, rhs: f64) {
        self.global += rhs;
        self.subwave += rhs;
        self.wave += rhs;
    }
}

impl fmt::Display for Timer{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Timer: timestamp: {}, \n
global: {}, \n
fragment: {}, \n
wave: {}",
        self.timestamp, self.global, self.subwave, self.wave)
    }
}