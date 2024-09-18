pub(super) mod tile;

use crate::unit::enemy::{Enemy, EnemyShared};
use crate::unit::operator::OperatorShared;
use crate::unit::scope::Scope;
use crate::utils::math::{distance_p2p, Grid};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use tile::PASS_ALL;

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
    pub(super) enemy: Vec<Vec<Vec<EnemyShared>>>,
    pub(super) operator: Vec<Vec<OperatorShared>>,
}
impl Map {
    pub(super) fn new(v: &Value) -> Result<Map> {
        let mut m: Map = from_value(v.clone())?;
        m.enemy = vec![vec![Vec::<EnemyShared>::new(); m.width as usize]; m.height as usize];
        m.operator = vec![vec![OperatorShared::new(); m.width as usize]; m.height as usize];
        generate_layout(v, m)
        // Ok(m)
    }
    pub(crate) fn update_enemy_map(&mut self, enemy_set: Vec<Rc<RefCell<Enemy>>>) {
        self.enemy
            .iter_mut()
            .for_each(|v| v.iter_mut().for_each(|v| v.clear()));
        for er in enemy_set {
            let e = er.borrow();
            let center = e.location;
            let ul = (center.x as u32, center.y as u32);
            // no boundary check because enemy location might be legal
            let ur = (ul.0 + 1, ul.1);
            let dl = (ul.0, ul.1 + 1);
            let dr = (ur.0, dl.1);
            self.enemy[(ul.1) as usize][(ul.0) as usize].push(Rc::downgrade(&er));
            // up-left point
            if ul.0 != 0 && ul.1 != 0 {
                if distance_p2p(&center, &ul.into()) <= ENEMY_TOUCH_SIZE {
                    self.enemy[(ul.1 - 1) as usize][(ul.0 - 1) as usize].push(Rc::downgrade(&er));
                }
            }
            // up-right point
            if ur.0 != self.width && ur.1 != 0 {
                if distance_p2p(&center, &ur.into()) <= ENEMY_TOUCH_SIZE {
                    self.enemy[(ur.1 - 1) as usize][(ur.0) as usize].push(Rc::downgrade(&er));
                }
            }
            // down-left point
            if dl.0 != 0 && dl.1 != self.height {
                if distance_p2p(&center, &dl.into()) <= ENEMY_TOUCH_SIZE {
                    self.enemy[(dl.1) as usize][(dl.0 - 1) as usize].push(Rc::downgrade(&er));
                }
            }
            // down-right point
            if dr.0 != self.width && dr.1 != self.height {
                if distance_p2p(&center, &dr.into()) <= ENEMY_TOUCH_SIZE {
                    self.enemy[(dr.1) as usize][(dr.0) as usize].push(Rc::downgrade(&er));
                }
            }
            // up point
            if ul.1 != 0 {
                if center.y - ul.1 as f64 <= ENEMY_TOUCH_SIZE {
                    self.enemy[(ul.1 - 1) as usize][(ul.0) as usize].push(Rc::downgrade(&er));
                }
            }
            // down point
            if dl.1 != self.height {
                if dl.1 as f64 - center.y <= ENEMY_TOUCH_SIZE {
                    self.enemy[(dl.1) as usize][(dl.0) as usize].push(Rc::downgrade(&er));
                }
            }
            // left point
            if ul.0 != 0 {
                if center.x - ul.0 as f64 <= ENEMY_TOUCH_SIZE {
                    self.enemy[(ul.1) as usize][(ul.0 - 1) as usize].push(Rc::downgrade(&er));
                }
            }
            // right point
            if ur.0 != self.width {
                if ur.0 as f64 - center.x <= ENEMY_TOUCH_SIZE {
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
    pub(crate) fn search_enemy(&self, search_scope: &Scope) -> Vec<EnemyShared> {
        let mut ve = Vec::<Weak<RefCell<Enemy>>>::new();
        match search_scope {
            Scope::Grids(rect) => {
                for r in rect.iter() {
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
            }
            Scope::Circle(_, _) => {
                todo!()
            }
            Scope::None => {}
        }
        ve
    }
    pub(crate) fn search_operator(&self, search_scope: &Scope) -> Vec<OperatorShared> {
        let mut vo = Vec::new();
        match search_scope {
            Scope::Grids(_rect) => {
                todo!();
            }
            Scope::Circle(center, r) => {
                let center = center;
                let r_squared = r * r;
                let start_row = (center.y - r) as usize;
                let end_row = (center.y + r) as usize;
                let start_col = (center.x - r) as usize;
                let end_col = (center.x + r) as usize;
                for i in start_row..=end_row {
                    for j in start_col..=end_col {
                        let distance_squared = distance_p2p(&(i, j).into(), &center);
                        if distance_squared <= r_squared {
                            if let Some(_o) = self.operator[i][j].upgrade() {
                                vo.push(self.operator[i][j].clone());
                            }
                        }
                    }
                }
            }
            Scope::None => {}
        }
        vo
    }
    pub(super) fn update_layout(&mut self) {
        unimplemented!("update layout")
    }
    fn grid_in_map(&self, p: &Grid) -> bool {
        p.row >= 0 && p.row < self.height as i64 && p.col >= 0 && p.col < self.width as i64
    }

    fn grid_can_pass(&self, p: &Grid) -> bool {
        self.grid_in_map(p) && (self.layout[p.row as usize][p.col as usize] | PASS_ALL == PASS_ALL)
    }

    pub(super) fn spfa(&self, start: Grid, end: Grid) -> Vec<Grid> {
        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut dis = vec![vec![std::u32::MAX; self.width as usize]; self.height as usize];
        let mut path = vec![vec![start; self.width as usize]; self.height as usize];
        dis[start.row as usize][start.col as usize] = 0;
        let dir = vec![
            Grid { row: 0, col: 1 },
            Grid { row: 0, col: -1 },
            Grid { row: 1, col: 0 },
            Grid { row: -1, col: 0 },
        ];
        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            for d in dir.iter() {
                let next = cur + *d;
                if !self.grid_can_pass(&next) {
                    continue;
                }
                if dis[next.row as usize][next.col as usize]
                    > dis[cur.row as usize][cur.col as usize] + 1
                {
                    dis[next.row as usize][next.col as usize] =
                        dis[cur.row as usize][cur.col as usize] + 1;
                    path[next.row as usize][next.col as usize] = cur;
                    if queue.iter().all(|p| *p != next) {
                        queue.push_back(next);
                    }
                }
            }
        }
        let mut route = Vec::new();
        let mut p = end;
        while p != start {
            route.push(p);
            p = path[p.row as usize][p.col as usize];
        }
        route
    }
}
