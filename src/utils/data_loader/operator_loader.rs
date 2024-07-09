use serde::{Deserialize, Serialize};
use serde_json::{from_str, from_value, Value};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use log::error;
use crate::unit::scope::Scope;
use crate::unit::skill::effect::DamageType;
use crate::unit::skill::skill_type::{AttackType, ChargeType};
use crate::utils::math::Grid;
use crate::unit::operator::Operator;
use crate::unit::{Unit, UnitInfo};
use crate::unit::skill::Skill;
use crate::unit::skill::skill_type::TriggerType;
use crate::utils::load_json_file;
use crate::utils::math::GridRect;
use super::Result;
use super::Loader;

#[derive(Deserialize,Default,Debug)]
struct OfficalOperator{
    pub(super) name:String,
    pub(super) displayNumber:String,
    pub(super) appellation:String,
    pub(super) phases:Vec<OfficalPhase>,
    pub(super) skills:Vec<OfficalSkillsDescription>,
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
    pub(super) maxHp:i64,
    pub(super) atk:i64,
    pub(super) def:i64,
    pub(super) magicResistance:f64,
    pub(super) cost:i64,
    pub(super) blockCnt: i64,
    pub(super) moveSpeed: f64,
    pub(super) attackSpeed: f64,
    pub(super) baseAttackTime: f64,
    pub(super) respawnTime: i64,
    pub(super) hpRecoveryPerSec: f32,
    pub(super) spRecoveryPerSec: f32,
    pub(super) maxDeployCount: f32,
    pub(super) maxDeckStackCnt: f32,
    pub(super) tauntLevel: i64,
    pub(super) massLevel: i64,
    pub(super) baseForceLevel: i64,
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
struct OfficalSkillsDescription{
    pub(super) skillId:String,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalSkill{
    rangeId:String,
    skillType:String,
    durationType:String,
    duration:f64,
    spData:OfficalSpData,
    blackboard:Vec<OfficalBlackBoard>,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalSpData{
    spType:String,
    levelUpCost:Option<u32>,
    maxChargeTime:u32,
    spCost:u32,
    initSp:u32,
    increment:f32,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalBlackBoard{
    key:String,
    value:f64,
    valueStr:Option<String>,
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
            "AUTO"=>TriggerType::Auto,
            "MANUAL"=>TriggerType::Manual,
            "PASSIVE"=>TriggerType::Passive,
            _=>TriggerType::None
        };
        let charge_type:ChargeType = match self.spData.spType.as_str(){
            "INCREASE_WITH_TIME"=>ChargeType::Time,
            "INCREASE_WITH_ATTACK"=>ChargeType::Attack,
            "INCREASE_WITH_BE_HIT"=>ChargeType::BeHit,
            _=>ChargeType::None
        };
        Skill{
            duration:self.duration,
            sp:self.spData.initSp as f64,
            sp_cost:self.spData.spCost as f64,
            trigger_type,
            charge_type,
            ..Default::default()
        }
    }
}

impl Loader{
    /// name can be english or chinese, if name is english,first letter should be upper case
    /// phase is the phase of operator, 0 is the lowest phase, 2 is the highest phase
    /// level is the level of operator, 1 is the lowest level, the highest level depend on phase and operator
    /// skill level is the level of skill, 1 is the lowest level, the highest level depend phase and operator
    /// return None if operator not found or phase or level is wrong
    fn load_operator(&self,name:String,phase:usize,level:u32,skill_index:usize,skill_level:usize)->Result<Operator>{
        let ok = self.get_operator_key(&name).ok_or("Operator not found")?;
        let oo= from_value::<OfficalOperator>(self.character_table[ok].clone())?;
        let mut o=self.operator_phase_generate(name,phase,level,skill_index,skill_level,&oo)?;
        let sp=from_value::<DamageType>(self.gamedata_const["subProfessionDamageTypePairs"][oo.subProfessionId.clone()].clone())?;
        o.info.damage_type=sp;
        o.stage.damage_type=sp;
        return Ok(o);
    }
    fn operator_phase_generate(&self,name:String,phase:usize,level:u32,skill_index:usize,skill_level:usize,oo:&OfficalOperator)->Result<Operator>{
        let op = oo.phases.get(phase).ok_or("Phase not found")?;
        let max_level =op.maxLevel;
        let max_skill_level = match phase{
            0=>4,
            1=>7,
            2=>10,
            _=>0
        }; 
        if level >= 1 && level <= max_level && skill_level >= 1 && skill_level <= max_skill_level {
            let mut r= from_value::<OfficalRange>(self.range_table[op.rangeId.clone()].clone())?;
            let mut at= from_value::<AttackType>(Value::String(oo.position.clone()))?;
            let sd=oo.skills.get(skill_index-1).ok_or("Skill not found")?;
            let mut s=self.operator_skill_generate(sd.skillId.clone(),skill_level-1)?;
            let mut o =Operator::default();
            let upper = &op.attributesKeyFrames[1].data;
            let mut data = op.attributesKeyFrames[0].data.clone();
            let change = (level - 1) as f64 / (max_level - 1) as f64;
            data.maxHp += ((upper.maxHp - data.maxHp) as f64 * change) as i64;
            data.atk += ((upper.atk - data.atk) as f64  * change) as i64;
            data.def += ((upper.def - data.def) as f64 * change) as i64;
            let mut ui: UnitInfo = data.into();
            ui.attack_type = at;
            let s = Scope { 0: r.merge() };
            o.attack_scope = s.clone();
            o.search_scope = s;
            o.re_deploy=upper.respawnTime as f32;
            o.info=ui.clone();
            o.stage=ui;
            o.name=name;
            return  Ok(o);
        }else{
            Err("Level or skill level out of range,max_level is {max_level},max_skill_level is {max_skill_level}".into())
        }
    }

    fn operator_skill_generate(&self,skill_id:String,skill_level:usize)->Result<Skill>{
        let os=from_value::<OfficalSkill>(self.skill_table[skill_id]["levels"][skill_level].clone())?;
        let mut s:Skill = os.into();
        Ok(s)
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
            let oo=l.load_operator("Shu".into(),2,30,2,8).unwrap();
            println!("{:?}",oo)  ;
        }else{
            println!("wrong data path in loader test");
        }
    }
}