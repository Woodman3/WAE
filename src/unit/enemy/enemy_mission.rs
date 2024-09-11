use crate::{
    calculator::PERIOD, event::Event, frame::Frame, route::CheckPoint, unit::code, utils::math
};

use super::Enemy;

impl Enemy {
    pub(crate) fn arrange_mission(&mut self) {
        self.mission_vec.push(Self::step_mission);
        self.mission_vec.push(Self::skill_mission);
    }
    /// t is 1/fps it mean time interval
    fn step_mission(&mut self, f: &mut Frame) {
        if !matches!(self.be_block.upgrade(), None) {
            return;
        }
        let next_point;
        if self.route_stage == self.route.checkpoints.len() {
            next_point = self.route.end;
        }else if self.route_stage < self.route.checkpoints.len() {
            match &mut self.route.checkpoints[self.route_stage]{
                CheckPoint::Move(p) => {
                    next_point = *p;
                }
                CheckPoint::WaitForSeconds(t) => {
                    if *t>0.0 {
                        *t -= PERIOD;
                    }else{
                        self.route_stage += 1;
                    }
                    return;
                }
                _ => { return }

            }
        }else{
            f.events.push(Event::EnemyEnterEvent(self.id)) ;
            return ;
        }
        let (direction, new) = math::to_target(self.location, next_point, self.move_speed);
        let distance = math::distance_from_segment_to_point(self.location, new, next_point);
        if distance <= code::MIN_DISTANCE {
            self.route_stage += 1;
        }
        self.direction = direction;
        self.location = new;
    }

    fn skill_mission(&mut self, f: &mut Frame) {
        let sv = self.skills.step(f);
        for s in sv.iter() {
            s.shoot(f, self.location.into());
        }
        self.skills.skill_block.extend(sv);
    }
}
