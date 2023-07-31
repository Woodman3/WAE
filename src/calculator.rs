use crate::timeline::Event;
use crate::frame::Frame;
use crate::utils::config::Config;
use std::collections::VecDeque;
type Result<T> = std::result::Result<T,Box<dyn std::error::Error>>;
/// calculate
pub struct Calculator{
    frame_vec:Vec<Frame>,
    /// star in the battle we will get
    /// -1 mean battle haven't end
    star:i8,
    /// first element refer to time 
    /// second element refer to event vector
    time_line:VecDeque<(usize,usize)>,
    event_vec:Vec<Box<dyn Event>>,
    pub route:Vec<Vec<(f64,f64)>>,
    time_remain:i64
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
            self.event(&mut f);
            self.frame_vec.push(f);
            true
        }else {
            false
        }

    }
    /// an event is place event or enemy comeout or something happen like fire rain
    /// mosttime is happen in an specify time but sometime it happen after somethine has happen
    /// it can't be skip
    fn event(&mut self,f: &mut Frame){
        let time_stamp = self.frame_vec.len();
        while self.time_line.len()!=0{
            if let Some((time,e)) = self.time_line.pop_front(){
                if time != time_stamp{
                    if time < time_stamp{
                        println!("Some event not happened before,this event has drop");
                        continue;
                    }else {
                        break;
                    }
                }else{
                    self.event_vec[e].happen(f,self);
                }
            }
        }
        // for (time,e) in self.time_line.iter(){
        //     if *time != time_stamp{
        //         if *time < time_stamp{
        //             println!("Some event not happened before,this event has drop");
        //             continue;
        //         }else {
        //             break;
        //         }
        //     }else{
        //         self.event_vec[*e].happen(f,self);
        //     }
        // }
    }
    pub fn new(c:&Config)->Result<Calculator>{
        use crate::timeline::hostile::EnemyPlaceEvent;
        use crate::unit::enemy::Enemy;
        let mut event_vec = Vec::<Box::<dyn Event>>::new(); 
        let mut time_line = VecDeque::<(usize,usize)>::new();        
        let time_remain:i64=c.hostile["time_remain"].as_i64().unwrap();
        let mut route = Vec::<Vec::<(f64,f64)>>::new();
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
            route.push(r);
        }
        for v in c.hostile["event"].as_array().unwrap(){
            let e:EnemyPlaceEvent=serde_json::from_value(v.clone())?;
            event_vec.push(Box::new(e));
        }
        let mut frame_vec=Vec::<Frame>::new();
        frame_vec.push(Frame{
            enemy_position:Vec::<(f64,f64)>::new(),
            enemy_set:Vec::<Enemy>::new()
        });
        Ok(Calculator{
            frame_vec,
            star:-1,
            time_line,
            event_vec,
            route,
            time_remain
        })
    }
    pub fn to_end(&mut self){
        while self.next() {

        }
        println!("{:?}",self.frame_vec.last());
    }

    // fn enemy_move(&self,f:mut &Frame){
    // }
}

