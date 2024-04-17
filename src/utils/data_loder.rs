use crate::utils::math::Grid;
use serde::{Deserialize, Serialize};
use serde_json::{Value,from_value};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use log::error;
use crate::unit::operator::Operator;
use crate::unit::{Unit, UnitInfo};

use super::error;
use super::math::GridRect;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
struct Loader{
    character_table:Value,
    range_table:Value
}


#[derive(Deserialize,Default,Debug)]
struct OfficalOperator{
    pub(super) name:String,
    pub(super) displayNumber:String,
    pub(super) appellation:String,
    pub(super) position:String,
    pub(super) phases:Vec<OfficalPhase>,
    pub(super) skills:Vec<OfficalSkill>,

}
#[derive(Deserialize,Default,Debug)]
struct OfficalPhase{
    pub(super) rangeId:String,
    pub(super) maxLevel:u32,
    pub(super) attributesKeyFrames:Vec<OfficalKeyFrame>
}
#[derive(Deserialize,Default,Debug)]
struct OfficalKeyFrame{
    pub(super) level:u32,
    pub(super) data:OfficalData 
}
#[derive(Deserialize,Default,Debug)]
struct OfficalData{
    pub(super) maxHp:u32,
    pub(super) atk:u32,
    pub(super) def:u32,
    pub(super) magicResistance:f64,
    pub(super) cost:u32,
    pub(super) blockCnt: u32,
    pub(super) moveSpeed: f32,
    pub(super) attackSpeed: f64,
    pub(super) baseAttackTime: f64,
    pub(super) respawnTime: u32,
    pub(super) hpRecoveryPerSec: f32,
    pub(super) spRecoveryPerSec: f32,
    pub(super) maxDeployCount: f32,
    pub(super) maxDeckStackCnt: f32,
    pub(super) tauntLevel: u32,
    pub(super) massLevel: u32,
    pub(super) baseForceLevel: u32,
    pub(super) stunImmune: bool,
    pub(super) silenceImmune: bool,
    pub(super) sleepImmune: bool ,
    pub(super) frozenImmune: bool,
    pub(super) levitateImmune: bool,
    pub(super) disarmedCombatImmune: bool
}
#[derive(Deserialize,Default,Debug)]
struct OfficalSkill{
    skillId:String,
}

struct OfficalRange{
    grids:Vec<Grid>,
}

impl Into<UnitInfo> for OfficalData{
    fn into(self) -> UnitInfo {
        UnitInfo{
            hp: self.maxHp ,
            max_hp: self.maxHp,
            aspd: self.attackSpeed,
            atk: self.atk,
            def: self.def,
            magic_resist: self.magicResistance,
            attack_time: self.baseAttackTime,
            block_num: self.blockCnt,
            //damage_type and attack_type should init by other way
            ..Default::default()
        }
    }
}

impl Loader{
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Loader> {
        let mut file = File::open(path.as_ref().join("character_table.json"))?;
        let mut reader = BufReader::new(file);
        let character_table=serde_json::from_reader(reader)?;
        file = File::open(path.as_ref().join("range_table.json"))?;
        reader = BufReader::new(file);
        let range_table=serde_json::from_reader(reader)?;
        Ok(Loader{character_table,range_table})
    }
    fn operator_loader(&self,name:String,phase:usize,level:u32)->Option<Operator>{
        if let Some(ok) = self.get_operator_key(&name){
            if let Ok(oo)= from_value::<OfficalOperator>(self.character_table[ok].clone()){
                if let Some(ui) =self.operator_phase_generate(name,phase,level,&oo){
                    let mut o = Operator::default();
                    o.info =ui;
                    o.stage=ui; 
                    return Some(o)
                }
            }
        }
        None
        
    }
    fn operator_phase_generate(&self,name:String,phase:usize,level:u32,oo:&OfficalOperator)->Option<UnitInfo>{
        if let Some(op) = oo.phases.get(phase){
            let max_level =op.maxLevel;
            if level<max_level && level>0 {
                let upper = op.attributesKeyFrames[1].data;
                let mut data = op.attributesKeyFrames[0].data; 
                let change  = level as f32 /max_level as f32; 
                data.maxHp+=(upper.maxHp-data.maxHp)*change;
                data.atk+=(upper.atk-data.atk)*change;
                data.def+=(upper.def-data.def)*change;
                let mut ui:UnitInfo = data.into(); 
                if let Ok(r)= from_value::<OfficalRange>(self.range_table[op.rangeId]){
                    return  Some(ui) 
                }
            }
        }else{
            error!("wrong phase level of {name}");
        }
        None
    }

    pub(super) fn get_operator_key(&self,name:&String)->Option<String>{
        for (k,v) in self.character_table.as_object().unwrap(){
            let en =v["appellation"].as_str().unwrap();
            let cn =v["name"].as_str().unwrap();
            let nr=name.as_str();
            if(nr==en||nr==cn){
                return Some(k.clone());
            }
        }
        None
    }
}

impl OfficalRange{
    pub(super) fn shorter(&mut self)->Vec<GridRect>{
        let mut r = Vec::<GridRect>::new();
        let v = &mut self.grids;
        v.sort_by(|a,b| {
            if(a.col!=b.col){
                a.col.cmp(&b.col)
            }else{
                a.row.cmp(&b.row)
            }
        });
        let mut i = 0;
        while i < v.len() {
            let s = v[i];
            let mut gr = GridRect { ul: s, dr: s };
            while i + 1 < v.len() && v[i + 1].row == gr.dr.row+1 && v[i + 1].col == gr.dr.col {
                gr.dr.row += 1;
                i += 1;
            }
            r.push(gr);
            i += 1;
        }
        
        let mut merged = Vec::<GridRect>::new();
        for gr in r {
            if let Some(last) = merged.last_mut() {
                if last.dr.row == gr.dr.row&&last.ul.row==gr.dr.row && last.dr.col + 1 == gr.ul.col {
                    last.dr.col = gr.dr.col; // 合并 GridRect
                    continue;
                }
            }
            merged.push(gr);
        }
        merged
    }
}

#[cfg(test)]
mod test{
    use super::Loader;
    #[test]
    fn loader_test(){
        if let Ok(l)=Loader::new("./data"){
            
            if let Some(oo)=l.operator_loader("Skadi".into(),1){
                println!("{:?}",oo)  ;
            }
        }else{
            println!("wrong data path in loader test");
        }
    }
}