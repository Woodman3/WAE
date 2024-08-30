use crate::{
    calculator::PERIOD,
    frame::Frame,
    timeline::{hostile::EnemyPlaceEvent, Event},
};

#[derive(Clone, Debug, Default)]
pub(super) struct Spawner {
    pub(super) wave: Vec<Wave>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct Wave {
    pub(super) pre_delay: f32,
    pub(super) wave: Vec<SubWave>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct SubWave {
    pub(super) pre_delay: f32,
    pub(super) wave: Vec<SubSubWave>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct SubSubWave {
    pub(super) count: i32,
    pub(super) interval: f32,
    pub(super) enemy: String,
    pub(super) route: u32,
    pub(super) pre_delay: f32,
    pub(super) cur_delay: f32,
    pub(super) cur_count: i32,
    pub(super) cur_interval: f32,
}

impl Spawner {
    pub(super) fn step(&mut self, f: &Frame) -> Vec<Event> {
        let mut ret = Vec::new();
        if let Some(mut w) = self.wave.pop() {
            ret.extend(w.step(f));
            if !w.finished() {
                self.wave.push(w);
            }
        }
        ret
    }
}

impl Wave {
    fn step(&mut self, f: &Frame) -> Vec<Event> {
        let mut ret = Vec::new();
        if self.pre_delay > 0.0 {
            self.pre_delay -= PERIOD as f32;
            return ret;
        }
        if let Some(mut sw) = self.wave.pop() {
            ret.extend(sw.step(f));
            if !sw.finished() {
                self.wave.push(sw);
            }
        }
        ret
    }

    fn finished(&self) -> bool {
        self.pre_delay <= 0.0 && self.wave.len() == 0
    }
}

impl SubWave {
    fn step(&mut self, f: &Frame) -> Vec<Event> {
        let mut ret = Vec::new();
        if self.pre_delay > 0.0 {
            self.pre_delay -= PERIOD as f32;
            return ret;
        }
        for w in self.wave.iter_mut() {
            ret.extend(w.step(f));
        }
        ret
    }
    fn finished(&self) -> bool {
        self.pre_delay <= 0.0 && self.wave.len() == 0
    }
}

impl SubSubWave {
    fn step(&mut self, f: &Frame) -> Vec<Event> {
        let mut ret = Vec::new();
        if self.cur_count == 0 {
            if self.cur_delay > 0.0 {
                self.cur_delay -= PERIOD as f32;
                ret
            } else {
                self.cur_count += 1;
                self.spawn(f)
            }
        } else {
            if self.cur_interval > 0.0 {
                self.cur_interval -= PERIOD as f32;
                ret
            } else {
                self.cur_count += 1;
                self.cur_interval = self.interval;
                self.spawn(f)
            }
        }
    }
    fn spawn(&mut self, f: &Frame) -> Vec<Event> {
        let e = EnemyPlaceEvent {
            enemy_key: self.enemy.clone(),
            enemy_route: self.route as usize,
        };
        vec![Event::EnemyPlaceEvent(e)]
    }
}
