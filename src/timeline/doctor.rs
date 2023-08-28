use super::Event;
use crate::calculator::Calculator;
use crate::frame::Frame;
use crate::unit::operator::Operator;
use crate::unit::scope::Toward;

#[derive(Debug,Clone)]
pub struct OperatorDeployEvent{
    operator_key:String,
    location:(u32,u32),
    toward:Toward,
}

impl OperatorDeployEvent{

}
impl Event for OperatorDeployEvent {
    fn happen(&self, f: &mut Frame, c: &Calculator) {
        let mut o :Operator=f.operator_undeploy.remove(&self.operator_key).unwrap();
        match self.toward {
            Toward::North => {}
            Toward::South => {}
            Toward::East => {}
            Toward::West => {}
        }
        o.toward=self.toward.clone();
        o.location=self.location;
        f.operator_deploy.insert(self.operator_key.clone(),o);
    }
}