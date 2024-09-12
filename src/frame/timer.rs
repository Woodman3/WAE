use std::{ops::{AddAssign},fmt};

use serde::{Deserialize, Serialize};

use crate::calculator::PERIOD;



#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub(crate) struct Timer{
    pub(crate) timestamp: u64,
    pub(crate) global: f32,
    pub(crate) wave: f32,
    pub(crate) subwave: f32,
}

impl Timer{
    pub(crate) fn step(&mut self){
        self.timestamp += 1;
        *self+=PERIOD;
    }
}

impl AddAssign<f32> for Timer{
    fn add_assign(&mut self, rhs: f32) {
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