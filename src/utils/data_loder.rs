use crate::unit::scope::Scope;
use crate::unit::skill::effect::DamageType;
use crate::unit::skill::skill_type::AttackType;
use crate::utils::math::Grid;
use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, from_value, Value};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use log::error;
use crate::unit::operator::Operator;
use crate::unit::{Unit, UnitInfo};
use crate::unit::skill::Skill;
use crate::unit::skill::skill_type::TriggerType;
use super::load_json_file;

use super::error;
use super::math::GridRect;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
struct Loader{
    character_table:Value,
    range_table:Value,
    gamedata_const:Value,
    skill_table:Value
}


#[derive(Deserialize,Default,Debug)]
struct OfficalOperator{
    pub(super) name:String,
    pub(super) displayNumber:String,
    pub(super) appellation:String,
    pub(super) phases:Vec<OfficalPhase>,
    pub(super) skills:Vec<OfficalSkill>,
    pub(super) subProfessionId:String,
    pub(super) position:String,
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
#[derive(Deserialize,Default,Debug,Clone)]
struct OfficalData{
    pub(super) maxHp:u32,
    pub(super) atk:u32,
    pub(super) def:u32,
    pub(super) magicResistance:f64,
    pub(super) cost:u32,
    pub(super) blockCnt: u32,
    pub(super) moveSpeed: f32,
    pub(super) attackSpeed: f64,
    pub(super) baseAttackTime: f32,
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
    pub(super) disarmedCombatImmune: bool,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalRange{
    grids:Vec<Grid>,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalSkill{
    rangeId:String,
    skillType:String,
    durationType:String,
    duration:f32,
    spData:OfficalSpData,
    blackBoard:Vec<OfficalBlackBoard>,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalSpData{
    spType:String,
    levelUpCost:u32,
    maxChargeTime:u32,
    spCost:u32,
    initSp:u32,
    increment:u32,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalBlackBoard{
    key:String,
    value:f32,
    valueStr:String,
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

impl Into<Skill> for OfficalSkill{
    fn into(self) -> Skill {
        let trigger_type:TriggerType = match self.skillType.as_str(){
            "AUTO"=>TriggerType::AUTO,
            "MANUAL"=>TriggerType::MANUAL,
            "PASSIVE"=>TriggerType::PASSIVE,
            _=>TriggerType::None
        };
        Skill{
            duration:self.duration,
            sp:self.spData.initSp as f32,
            sp_cost:self.spData.spCost as f32,
            trigger_type,
            charge_type:from_value(Value::String(self.spData.spType)).unwrap(),
            ..Default::default()
        }
    }
}

impl Loader{
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Loader> {
        let character_table = load_json_file(path.as_ref().join("character_table.json"))?;
        let range_table = load_json_file(path.as_ref().join("range_table.json"))?;
        let gamedata_const = load_json_file(path.as_ref().join("gamedata_const.json"))?;
        let skill_table = load_json_file(path.as_ref().join("skill_table.json"))?;

        Ok(Loader {
            character_table,
            range_table,
            gamedata_const,
            skill_table,
        })
    }
    fn operator_loader(&self,name:String,phase:usize,level:u32)->Option<Operator>{
        if let Some(ok) = self.get_operator_key(&name){
            if let Ok(oo)= from_value::<OfficalOperator>(self.character_table[ok].clone()){
                if let Some(mut o)=self.operator_phase_generate(name,phase,level,&oo){
                    if let Ok(sp)=from_value::<DamageType>(self.gamedata_const["subProfessionDamageTypePairs"][oo.subProfessionId.clone()].clone()){
                        o.info.damage_type=sp;
                        o.stage.damage_type=sp;
                        return Some(o);
                    }
                }
            }
        }
        None
        
    }
    fn operator_phase_generate(&self,name:String,phase:usize,level:u32,oo:&OfficalOperator)->Option<Operator>{
        if let Some(op) = oo.phases.get(phase){
            let max_level =op.maxLevel;
            if level<max_level && level>0 {
                if let Ok(mut r)= from_value::<OfficalRange>(self.range_table[op.rangeId.clone()].clone()){
                    if let Ok(mut at) = from_value::<AttackType>(Value::String(oo.position.clone())){
                        let mut o =Operator::default();
                        let upper = &op.attributesKeyFrames[1].data;
                        let mut data = op.attributesKeyFrames[0].data.clone(); 
                        let change  = (level-1) as f32 /(max_level-1) as f32; 
                        data.maxHp+=((upper.maxHp-data.maxHp) as f32*change) as u32;
                        data.atk+=((upper.atk-data.atk) as f32*change) as u32;
                        data.def+=((upper.def-data.def) as f32*change) as u32;
                        let mut ui:UnitInfo = data.into(); 
                        ui.attack_type=at;
                        let s = Scope{0:r.merge()};
                        o.attack_scope = s.clone();
                        o.search_scope= s;
                        o.re_deploy=upper.respawnTime as f32;
                        o.info=ui.clone();
                        o.stage=ui;
                        o.name=name;
                        return  Some(o) 
                    }
                }
            }
        }else{
            error!("wrong phase level of {name}");
        }
        None
    }

    fn operator_skill_generata(&self,skillId:String)->Option<Skill>{
        if let Ok(os)=from_value::<OfficalSkill>(self.skill_table[skillId].clone()){
            let mut s:Skill = os.into();
            return Some(s);
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
    pub(super) fn merge(&mut self)->Vec<GridRect>{
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
                if last.dr.row == gr.dr.row&&last.ul.row==gr.ul.row{
                    if last.dr.col + 1 == gr.ul.col|| last.dr.col - 1 == gr.ul.col{
                        last.dr.col = gr.dr.col; // 合并 GridRect
                        continue;
                    }
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
            if let Some(oo)=l.operator_loader("Shu".into(),0,30){
                println!("{:?}",oo)  ;
            }
        }else{
            println!("wrong data path in loader test");
        }
    }
}