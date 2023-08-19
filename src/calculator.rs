use crate::timeline::Event;
use crate::frame::Frame;
use crate::utils::config::Config;
use crate::unit;
use std::collections::{VecDeque,HashMap};
use std::rc::Rc;
type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;
/// calculate
#[derive(Debug)]
pub struct Calculator{
    frame_vec:Vec<Frame>,
    /// star in the battle we will get
    /// -1 mean battle haven't end
    star:i8,
    /// first element refer to time 
    /// second element refer to event vector
    time_line:VecDeque<(usize,usize)>,
    event_vec:Vec<Box<dyn Event>>,
    pub route:Vec<Rc<Vec<(f64,f64)>>>,
    time_remain:i64,
    pub enemy_initial:HashMap<String,unit::enemy::Enemy>
}

impl Calculator{
    fn next(&mut self)->bool {
        if self.star != -1{
            return false;
        }
        if self.time_remain == 0 {
            self.star = 0;
            return false;
        }
        self.time_remain -=1;
        if let Some(f)=self.frame_vec.last(){
            let mut f=f.clone();
            f.timestamp+=1;
            self.process_frame(&mut f);
            self.frame_vec.push(f);
            true
        }else {
            false
        }

    }
    pub fn process_frame(&mut self,f: &mut Frame)
    {
        self.event(f);
        f.step(0.001);
    }
    pub fn new(c:&Config)->Result<Calculator>{
        use crate::timeline::hostile::EnemyPlaceEvent;
        use crate::unit::enemy::Enemy;
        let mut event_vec = Vec::<Box::<dyn Event>>::new(); 
        let mut time_line = VecDeque::<(usize,usize)>::new();        
        let time_remain:i64=c.hostile["time_remain"].as_i64().unwrap();
        let mut route = Vec::<Rc::<Vec::<(f64,f64)>>>::new();
        let temp:Vec<Vec<u64>> = serde_json::from_value(c.hostile["timeline"].clone()).unwrap();
        for v in temp{
            time_line.push_back((v[0] as usize,v[1] as usize));
        }
        let temp:Vec<Vec<Vec<f64>>>=serde_json::from_value(c.hostile["route"].clone()).unwrap();
        for v in temp{
            let mut r = Vec::<(f64,f64)>::new(); 
            for c in v{
                r.push((c[0],c[1]));
            }
            route.push(Rc::new(r));
        }
        for v in c.hostile["event"].as_array().unwrap(){
            let e:EnemyPlaceEvent=serde_json::from_value(v.clone())?;
            event_vec.push(Box::new(e));
        }
        let mut frame_vec=Vec::<Frame>::new();
        frame_vec.push(Frame{
            timestamp:0,
            enemy_set:Vec::<Enemy>::new()
        });
        let mut enemy_initial=HashMap::<String,unit::enemy::Enemy>::new();
        for (key,v) in c.enemy.as_object().unwrap(){
            enemy_initial.insert(key.clone(),Enemy::new(v));
        }
        Ok(Calculator{
            frame_vec,
            star:-1,
            time_line,
            event_vec,
            route,
            time_remain,
            enemy_initial
        })
    }
    pub fn to_end(&mut self){
        while self.next() {
            if let Some(f)=self.frame_vec.last(){
                println!("{}",f);

            }
        }
    }
    /// an event is place event or enemy comeout or something happen like fire rain
    /// mosttime is happen in an specify time but sometime it happen after somethine has happen
    /// it can't be skip
    fn event(&mut self,f: & mut Frame){
        let time_stamp = f.timestamp as usize;
        while self.time_line.len()!=0{
            if let Some((time,e)) = self.time_line.front(){
                if *time != time_stamp {
                    if *time < time_stamp {
                        println!("Some event not happened before,this event has drop");
                        self.time_line.pop_front();
                        continue;
                    }else {
                        break;
                    }
                }else{
                    self.event_vec[*e].happen(f,self);
                    self.time_line.pop_front();
                }
            }
        }
    }


    // fn enemy_move(&self,f:mut &Frame){
    // }
}

