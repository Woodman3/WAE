use crate::{
    calculator::PERIOD,
    event::{hostile::EnemyPlaceEvent, Event},
    frame::Frame,
};

#[derive(Clone, Debug, Default)]
pub(super) struct Spawner {
    pub(super) wave: Vec<Wave>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct Wave {
    pub(super) pre_delay: f64,
    pub(super) wave: Vec<SubWave>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct SubWave {
    pub(super) pre_delay: f64,
    pub(super) wave: Vec<SubSubWave>,
}

#[derive(Clone, Debug, Default)]
pub(super) struct SubSubWave {
    pub(super) count: i32,
    pub(super) interval: f64,
    pub(super) enemy: String,
    pub(super) route: u32,
    pub(super) pre_delay: f64,
    pub(super) cur_count: i32,
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
        // if self.pre_delay > 0.0 {
        //     self.pre_delay -= PERIOD;
        //     return ret;
        // }
        if let Some(mut sw) = self.wave.pop() {
            ret.extend(sw.step(f));
            if !sw.finished() {
                self.wave.push(sw);
            }
        }
        ret
    }

    fn finished(&self) -> bool {
        // self.pre_delay <= 0.0 && self.wave.len() == 0
        self.wave.len() == 0
    }
}

impl SubWave {
    fn step(&mut self, f: &Frame) -> Vec<Event> {
        let mut ret = Vec::new();
        // if self.pre_delay > 0.0 {
        //     self.pre_delay -= PERIOD;
        //     return ret;
        // }
        // for w in self.wave.iter_mut() {
        //     ret.extend(w.step(f));
        // }
        // ret
        if f.timer.subwave >= self.pre_delay {
            for w in self.wave.iter_mut() {
                ret.extend(w.step(f));
            }
            self.wave.retain(|w| !w.finished());
        }
        ret
    }
    fn finished(&self) -> bool {
        // self.pre_delay <= 0.0 && self.wave.len() == 0
        self.wave.len() == 0
    }
}

impl SubSubWave {
    fn step(&mut self, f: &Frame) -> Vec<Event> {
        let ret = Vec::new();
        let cur_time = f.timer.subwave;
        if cur_time >= self.pre_delay + self.cur_count as f64 * self.interval {
            self.cur_count+=1;
            return self.spawn(f)    
        }
        // if self.cur_count == 0 {
        //     if self.cur_delay > 0.0 {
        //         self.cur_delay -= PERIOD as f32;
        //         ret
        //     } else {
        //         self.cur_count += 1;
        //         self.spawn(f)
        //     }
        // } else {
        //     if self.cur_interval > 0.0 {
        //         self.cur_interval -= PERIOD as f32;
        //         ret
        //     } else {
        //         self.cur_count += 1;
        //         self.cur_interval = self.interval;
        //         self.spawn(f)
        //     }
        // }
        ret
    }
    fn spawn(&mut self, _f: &Frame) -> Vec<Event> {
        let e = EnemyPlaceEvent {
            enemy_key: self.enemy.clone(),
            enemy_route: self.route as usize,
        };
        vec![Event::EnemyPlaceEvent(e)]
    }

    fn finished(&self) -> bool {
        self.cur_count >= self.count
    }
}

impl PartialEq for SubSubWave {
    fn eq(&self, other: &Self) -> bool {
        self.pre_delay == other.pre_delay
    }
}

impl PartialOrd for SubSubWave {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.pre_delay.partial_cmp(&other.pre_delay)
    }
}