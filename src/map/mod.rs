pub(super) mod tile;

use crate::unit::enemy::Enemy;
use crate::unit::scope::Scope;
use crate::utils::math::distance_p2p;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use self::tile::generate_layout;

pub const ENEMY_TOUCH_SIZE: f64 = 0.15;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Map {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) layout: Vec<Vec<u64>>,
    #[serde(skip)]
    pub(super) enemy: Vec<Vec<Vec<Weak<RefCell<Enemy>>>>>,
    pub(super) operator: Vec<Vec<Option<String>>>,
}
impl Map {
    pub(super) fn new(v: &Value) -> Result<Map> {
        let mut m: Map = from_value(v.clone())?;
        m.enemy =
            vec![vec![Vec::<Weak<RefCell<Enemy>>>::new(); m.width as usize]; m.height as usize];
        m.operator = vec![vec![None; m.width as usize]; m.height as usize];
        generate_layout(v, m)
        // Ok(m)
    }
    pub(crate) fn update_enemy_map(&mut self, enemy_set: Vec<Rc<RefCell<Enemy>>>) {
        self.enemy
            .iter_mut()
            .for_each(|v| v.iter_mut().for_each(|v| v.clear()));
        for er in enemy_set {
            let e = er.borrow();
            let center: (f64, f64) = e.location.into();
            let ul = (center.0 as u32, center.1 as u32);
            // no boundary check because enemy location might be legal
            let ur = (ul.0 + 1, ul.1);
            let dl = (ul.0, ul.1 + 1);
            let dr = (ur.0, dl.1);
            self.enemy[(ul.1) as usize][(ul.0) as usize].push(Rc::downgrade(&er));
            // up-left point
            if ul.0 != 0 && ul.1 != 0 {
                if distance_p2p(&center, &ul) <= ENEMY_TOUCH_SIZE {
                    self.enemy[(ul.1 - 1) as usize][(ul.0 - 1) as usize].push(Rc::downgrade(&er));
                }
            }
            // up-right point
            if ur.0 != self.width && ur.1 != 0 {
                if distance_p2p(&center, &ur) <= ENEMY_TOUCH_SIZE {
                    self.enemy[(ur.1 - 1) as usize][(ur.0) as usize].push(Rc::downgrade(&er));
                }
            }
            // down-left point
            if dl.0 != 0 && dl.1 != self.height {
                if distance_p2p(&center, &dl) <= ENEMY_TOUCH_SIZE {
                    self.enemy[(dl.1) as usize][(dl.0 - 1) as usize].push(Rc::downgrade(&er));
                }
            }
            // down-right point
            if dr.0 != self.width && dr.1 != self.height {
                if distance_p2p(&center, &dr) <= ENEMY_TOUCH_SIZE {
                    self.enemy[(dr.1) as usize][(dr.0) as usize].push(Rc::downgrade(&er));
                }
            }
            // up point
            if ul.1 != 0 {
                if center.1 - ul.1 as f64 <= ENEMY_TOUCH_SIZE {
                    self.enemy[(ul.1 - 1) as usize][(ul.0) as usize].push(Rc::downgrade(&er));
                }
            }
            // down point
            if dl.1 != self.height {
                if dl.1 as f64 - center.1 <= ENEMY_TOUCH_SIZE {
                    self.enemy[(dl.1) as usize][(dl.0) as usize].push(Rc::downgrade(&er));
                }
            }
            // left point
            if ul.0 != 0 {
                if center.0 - ul.0 as f64 <= ENEMY_TOUCH_SIZE {
                    self.enemy[(ul.1) as usize][(ul.0 - 1) as usize].push(Rc::downgrade(&er));
                }
            }
            // right point
            if ur.0 != self.width {
                if ur.0 as f64 - center.0 <= ENEMY_TOUCH_SIZE {
                    self.enemy[(ur.1) as usize][(ur.0) as usize].push(Rc::downgrade(&er));
                }
            }
        }
    }
    pub(crate) fn deep_clone(&self) -> Self {
        let width = self.width;
        let height = self.height;
        let enemy = vec![vec![Vec::<Weak<RefCell<Enemy>>>::new(); width as usize]; height as usize];
        Map {
            width,
            height,
            layout: self.layout.clone(),
            enemy,
            operator: self.operator.clone(),
        }
    }
    pub(crate) fn search(&self, search_scope: &Scope) -> Vec<Weak<RefCell<Enemy>>> {
        let mut ve = Vec::<Weak<RefCell<Enemy>>>::new();
        for r in search_scope.0.iter() {
            for i in r.ul.row..=r.dr.row {
                for j in r.ul.col..=r.ul.col {
                    for e in self.enemy[i as usize][j as usize].iter() {
                        if let Some(e) = e.upgrade() {
                            if !ve.iter().any(|e2| {
                                if let Some(e2) = e2.upgrade() {
                                    e2 == e
                                } else {
                                    false
                                }
                            }) {
                                ve.push(Rc::downgrade(&e));
                            }
                        }
                    }
                }
            }
        }
        ve
    }
    pub(super) fn update_layout(&mut self) {}
}
