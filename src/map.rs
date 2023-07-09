use serde::{Deserialize,Serialize};
#[derive(Serialize,Deserialize,Debug)]
pub struct Map{
    pub width:u64,
    pub height:u64,
    pub layout:Vec<Vec<u64>>
}
impl Map {

}

