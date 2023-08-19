use std::fmt::Debug;
use crate::frame::Frame;
use crate::calculator::Calculator;
pub mod hostile;
pub trait Event:Debug {
        fn happen(&self,f:& mut Frame,c:& Calculator);
}
