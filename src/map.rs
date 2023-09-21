use crate::frame::Frame;
use crate::mul2d;
use crate::unit::enemy::Enemy;
use crate::utils::math::distance_p2p;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::rc::Rc;

const ENEMY_TOUCH_SIZE: f64 = 0.3;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug,Clone)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub layout: Vec<Vec<u64>>,
    pub enemy: Vec<Vec<Vec<Rc<RefCell<Enemy>>>>>,
}
impl Map {
    pub fn new(v: &Value) -> Result<Map> {
        let width= serde_json::from_value(v["width"].clone())?;
        let height=serde_json::from_value(v["height"].clone())?;
        let enemy = vec![vec![Vec::<Rc<RefCell<Enemy>>>::new();width as usize];height as usize];
        Ok(Map {
            width,
            height,
            layout: serde_json::from_value::<Vec<Vec<u64>>>(v["layout"].clone())?,
            enemy,
        })
    }
    pub fn update_enemy_map(&mut self,enemy_set:Vec<Rc<RefCell<Enemy>>>) {
        self.enemy.iter_mut().for_each(|v| v.iter_mut().for_each(|v| v.clear()));
        for er in enemy_set {
            let e = er.borrow();
            let center:(f64,f64)= e.location.into();
            let ul = (center.0 as u32, center.1 as u32);
            // no boundary check because enemy location might be legal
            let ur = (ul.0, ul.1 + 1);
            let dl = (ul.0 + 1, ul.1);
            let dr = (dl.0, ur.1);
            self.enemy[(ul.1)as usize][(ul.0)as usize].push(er.clone());
            // up-left point
            if ul.0 != 0 && ul.1 != 0 {
                if distance_p2p(&center, &ul) <= ENEMY_TOUCH_SIZE {
                    self.enemy[(ul.1-1) as usize][(ul.0-1) as usize].push(er.clone());
                }
            }
            // up-right point
            if ur.0 != self.width && ur.1 != 0{
                if distance_p2p(&center, &ur) <= ENEMY_TOUCH_SIZE{
                    self.enemy[(ur.1-1)as usize][(ur.0)as usize].push(er.clone());
                }
            }
            // down-left point
            if dl.0 != 0 && dl.1 != self.height {
                if distance_p2p(&center, &dl) <= ENEMY_TOUCH_SIZE{
                    self.enemy[(dl.1)as usize][(dl.0-1)as usize].push(er.clone());
                }
            }
            // down-right point
            if dr.0 != self.width && dr.1 != self.height {
                if distance_p2p(&center, &dr) <= ENEMY_TOUCH_SIZE{
                    self.enemy[(dr.1)as usize][(dr.0)as usize].push(er.clone());
                }
            }
            // up point
            if ul.1!=0{
                if center.1-ul.1 as f64 <= ENEMY_TOUCH_SIZE{
                    self.enemy[(ul.1-1)as usize][(ul.0)as usize].push(er.clone());
                }
            }
            // down point
            if dl.1!=self.height{
                if dl.1 as f64 -center.1<= ENEMY_TOUCH_SIZE{
                    self.enemy[(dl.1)as usize][(dl.0)as usize].push(er.clone());
                }
            }
            // left point
            if ul.0!=0{
                if center.0-ul.0 as f64 <=ENEMY_TOUCH_SIZE{
                    self.enemy[(ul.1)as usize][(ul.0-1)as usize].push(er.clone());
                }
            }
            // right point
            if ur.0!=self.width{
                if ur.0 as f64 -center.0<=ENEMY_TOUCH_SIZE{
                    self.enemy[(ur.1)as usize][(ur.0)as usize].push(er.clone());
                }
            }
        }
    }
    pub fn deep_clone(&self)->Self{
        let width=self.width;
        let height=self.height;
        let enemy = vec![vec![Vec::<Rc<RefCell<Enemy>>>::new();width as usize];height as usize];
        Map{
            width,
            height,
            layout:self.layout.clone(),
            enemy
        }
    }
}