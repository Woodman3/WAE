
use crate::{
    frame::Frame,
    unit::{
        code,
    },
    utils::math::{self, to_target},
};

use super::Enemy;

impl Enemy {
    fn skill_mission(&mut self, f: &mut Frame) {
        self.skills.step(f);
    }
    pub(crate) fn arrange_mission(&mut self) {
        self.mission_vec.push(Self::step_mission);
        self.mission_vec.push(Self::skill_mission);
    }
    /// t is 1/fps it mean time interval
    fn step_mission(&mut self, _f: &mut Frame) {
        if matches!(self.be_block.upgrade(), None) {
            return;
        }
        let (direction, new) = to_target(self.location, self.next_point, self.move_speed);
        let distance = math::distance_from_segment_to_point(self.location, new, self.next_point);
        if distance <= code::MIN_DISTANCE {
            self.route_stage += 1;
            if self.route_stage < self.route.checkpoints.len() {
                use crate::route::CheckPoint;
                self.next_point = self.route.end;
                for i in self.route_stage..self.route.checkpoints.len() {
                    match self.route.checkpoints[i] {
                        CheckPoint::Move(p) => {
                            self.next_point = p;
                            break;
                        }
                        _ => continue,
                    }
                }
            } else if self.route_stage == self.route.checkpoints.len() {
                self.next_point = self.route.end;
            } else {
                self.die_code = code::INTO_END;
            }
        }
        self.direction = direction;
        self.location = new;
    }
}
