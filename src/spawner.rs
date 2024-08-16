use crate::{calculator::PERIOD, frame::Frame};



struct Spawner{
    wave: Vec<Wave>,
}

struct Wave{
    pre_delay: f32,
    wave:Vec<SubWave>,
}

struct SubWave{
    pre_delay: f32,
    wave:Vec<SubSubWave>,
}

struct SubSubWave{
    count: i32,
    interval: f32,
    enemy: String,
    route: u32,
    pre_delay: f32,
}

impl Spawner{
    fn step(&mut self,f:&mut Frame){
        if let Some(mut w) =self.wave.pop(){
            w.step(f);            
            if !w.finished(){
                self.wave.push(w);
            }
        }
    }
}

impl Wave{
    fn step(&mut self,f:&mut Frame){
        if self.pre_delay>0.0{
            self.pre_delay-=PERIOD as f32;
            return;
        }
        if let Some(mut sw) = self.wave.pop(){
            sw.step(f);
            if !sw.finished(){
                self.wave.push(sw);
            }
        }
    }

    fn finished(&self)->bool{
        self.pre_delay<=0.0 && self.wave.len()==0
    }
}

impl SubWave{
    fn step(&mut self,f:&mut Frame){
        if self.pre_delay>0.0{
            self.pre_delay-=PERIOD as f32;
            return;
        }
        for w in self.wave.iter_mut(){
            w.step(f);
        }
    }

    fn finished(&self)->bool{
        self.pre_delay<=0.0 && self.wave.len()==0
    }
}

impl SubSubWave{
    fn step(&mut self,f:&mut Frame){
        if self.pre_delay>0.0{
            self.pre_delay-=PERIOD as f32;
            return;
        }else if self.interval>0{
            self.interval-=PERIOD as f32;
            return;
        }

    }
}

