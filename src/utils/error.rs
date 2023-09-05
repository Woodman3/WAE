use std::fmt::{Debug, Display, Formatter};
use std::error::Error;
pub struct ConfigParseError(pub String);
impl Display for ConfigParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{}",self.0)
    }
}
impl Debug for ConfigParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{}",self.0)
    }
}
impl Error for ConfigParseError {}



