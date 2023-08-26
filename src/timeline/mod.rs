use crate::calculator::Calculator;
use crate::frame::Frame;
use std::fmt::Debug;
pub mod hostile;
pub trait Event: Debug {
    fn happen(&self, f: &mut Frame, c: &Calculator);
}
