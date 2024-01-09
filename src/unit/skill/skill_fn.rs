use std::cell::RefCell;
use std::rc::Weak;
use crate::map::Map;
use crate::unit::enemy::Enemy;
use crate::unit::skill::ToEnemySkill;

impl ToEnemySkill{
    pub(crate) fn search(&mut self,m:&Map)->bool{
        if let Some(s)=&self.search_scope{
            let mut ve = m.search(s);
            if ve.len()>=self.target_num{
                self.target=ve.drain(0..self.target_num).collect();
            }else{
                self.target=ve.drain(..).collect();
            }
        }
        self.target.len()!=0
    }
}