use crate::frame::Frame;
use crate::calculator::Calculator;
pub mod hostile;
pub trait Event {
    fn happen<'a>(&self,f:&'a mut Frame,c:&'a Calculator) ->&'a Frame;
}
