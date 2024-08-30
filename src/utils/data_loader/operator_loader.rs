use super::Loader;
use super::Result;
use crate::unit::operator::Operator;
use crate::unit::scope::Scope;
use crate::unit::skill::effect::DamageType;
use crate::unit::skill::skill_type::TriggerType;
use crate::unit::skill::skill_type::{AttackType, ChargeType};
use crate::unit::skill::{Skill, SpData};
use crate::unit::UnitInfo;
use crate::utils::math::Grid;
use crate::utils::math::GridRect;
use serde::Deserialize;
use serde_json::{from_value, Value};

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct OfficialOperator {
    pub(super) name: String,
    pub(super) display_number: String,
    pub(super) appellation: String,
    pub(super) phases: Vec<OfficialPhase>,
    pub(super) skills: Vec<OfficialSkillsDescription>,
    pub(super) sub_profession_id: String,
    pub(super) position: String,
}
#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct OfficialPhase {
    pub(super) range_id: String,
    pub(super) max_level: u32,
    pub(super) attributes_key_frames: Vec<OfficialKeyFrame>,
}
#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct OfficialKeyFrame {
    pub(super) level: u32,
    pub(super) data: OfficialData,
}
#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct OfficialData {
    pub(super) max_hp: i64,
    pub(super) atk: i64,
    pub(super) def: i64,
    pub(super) magic_resistance: f64,
    pub(super) cost: i64,
    pub(super) block_cnt: i64,
    pub(super) move_speed: f64,
    pub(super) attack_speed: f64,
    pub(super) base_attack_time: f64,
    pub(super) respawn_time: i64,
    pub(super) hp_recovery_per_sec: f32,
    pub(super) sp_recovery_per_sec: f32,
    pub(super) max_deploy_count: f32,
    pub(super) max_deck_stack_cnt: f32,
    pub(super) taunt_level: i64,
    pub(super) mass_level: i64,
    pub(super) base_force_level: i64,
    pub(super) stun_immune: bool,
    pub(super) silence_immune: bool,
    pub(super) sleep_immune: bool,
    pub(super) frozen_immune: bool,
    pub(super) levitate_immune: bool,
    pub(super) disarmed_combat_immune: bool,
}

#[derive(Deserialize, Default, Debug)]
struct OfficialRange {
    grids: Vec<Grid>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct OfficialSkillsDescription {
    pub(super) skill_id: String,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct OfficialSkill {
    range_id: Option<String>,
    skill_type: String,
    duration_type: String,
    duration: f64,
    sp_data: OfficialSpData,
    blackboard: Vec<OfficialBlackBoard>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct OfficialSpData {
    sp_type: String,
    level_up_cost: Option<u32>,
    max_charge_time: u32,
    sp_cost: u32,
    init_sp: u32,
    increment: f32,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct OfficialBlackBoard {
    key: String,
    value: f64,
    value_str: Option<String>,
}

impl Into<UnitInfo> for OfficialData {
    fn into(self) -> UnitInfo {
        UnitInfo {
            hp: self.max_hp,
            max_hp: self.max_hp,
            aspd: self.attack_speed,
            atk: self.atk,
            def: self.def,
            magic_resist: self.magic_resistance,
            attack_time: self.base_attack_time,
            block_num: self.block_cnt,
            //damage_type and attack_type should init by other way
            cost: self.cost as i32,
            ..Default::default()
        }
    }
}

impl Into<Skill> for OfficialSkill {
    fn into(self) -> Skill {
        let trigger_type: TriggerType = match self.skill_type.as_str() {
            "AUTO" => TriggerType::Auto,
            "MANUAL" => TriggerType::Manual,
            "PASSIVE" => TriggerType::Passive,
            _ => TriggerType::None,
        };
        Skill {
            duration: self.duration,
            trigger_type,
            sp_data: self.sp_data.into(),
            ..Default::default()
        }
    }
}

impl Into<SpData> for OfficialSpData {
    fn into(self) -> SpData {
        SpData {
            sp_cost: self.sp_cost as f64,
            sp: self.init_sp as f64,
            charge_type: match self.sp_type.as_str() {
                "INCREASE_WITH_TIME" => ChargeType::Time,
                "INCREASE_WITH_ATTACK" => ChargeType::Attack,
                "INCREASE_WITH_BE_HIT" => ChargeType::BeHit,
                _ => ChargeType::None,
            },
            overcharge: false,
        }
    }
}

impl Loader {
    /// name can be english or chinese, if name is english,first letter should be upper case
    /// elite: 0 is the lowest phase, 2 is the highest phase
    /// level: 1 is the lowest level, the highest level depend on phase and operator
    /// skill level: 1 is the lowest level, the highest level depend phase and operator, if set to 0 , it mean the operator don't have skill
    /// return None if operator not found or phase or level is wrong
    pub(crate) fn load_operator(
        &self,
        name: String,
        elite: usize,
        level: u32,
        skill_index: usize,
        skill_level: usize,
    ) -> Result<Operator> {
        let ok = self.get_operator_key(&name).ok_or("Operator not found")?;
        let oo = from_value::<OfficialOperator>(self.character_table[ok].clone())?;
        let mut o =
            self.operator_phase_generate(name, elite, level, skill_index, skill_level, &oo)?;
        let sp = from_value::<DamageType>(
            self.gamedata_const["subProfessionDamageTypePairs"][oo.sub_profession_id.clone()]
                .clone(),
        )?;
        o.info.damage_type = sp;
        o.stage.damage_type = sp;
        o.init();
        return Ok(o);
    }
    // fn load_copilot_operator(&self,copilot:Copilot)->Result<Vec<Operator>>{
    //     todo!()
    // }
    fn operator_phase_generate(
        &self,
        name: String,
        elite: usize,
        level: u32,
        skill_index: usize,
        skill_level: usize,
        oo: &OfficialOperator,
    ) -> Result<Operator> {
        let op = oo.phases.get(elite).ok_or(format!(
            "elite set wrong, elite is {elite} , operator is {name}"
        ))?;
        let max_level = op.max_level;
        let max_skill_level = match elite {
            0 => 4,
            1 => 7,
            2 => 10,
            _ => 0,
        };
        if level >= 1 && level <= max_level && skill_level >= 1 && skill_level <= max_skill_level {
            let mut r = from_value::<OfficialRange>(self.range_table[op.range_id.clone()].clone())?;
            let at = from_value::<AttackType>(Value::String(oo.position.clone()))?;
            let _skill = if skill_index == 0 {
                Skill::default()
            } else {
                let sd = oo.skills.get(skill_index - 1).ok_or(format!(
                    "Skill not found, operator is {name} skill index is {skill_index}"
                ))?;
                self.operator_skill_generate(sd.skill_id.clone(), skill_level - 1)?
            };
            let mut o = Operator::default();
            let upper = &op.attributes_key_frames[1].data;
            let mut data = op.attributes_key_frames[0].data.clone();
            let change = (level - 1) as f64 / (max_level - 1) as f64;
            data.max_hp += ((upper.max_hp - data.max_hp) as f64 * change) as i64;
            data.atk += ((upper.atk - data.atk) as f64 * change) as i64;
            data.def += ((upper.def - data.def) as f64 * change) as i64;
            let mut ui: UnitInfo = data.into();
            ui.attack_type = at;
            let s = Scope::Grids(r.merge());
            o.info.scope = s.clone();
            o.stage.scope = s;
            o.re_deploy = upper.respawn_time as f32;
            o.info = ui.clone();
            o.stage = ui;
            o.name = name;
            // o.skills.skill_block.push(skill);
            return Ok(o);
        } else {
            return Err("Level or skill level out of range,max_level is {max_level},max_skill_level is {max_skill_level}".into());
        }
        todo!("skill not implement")
    }

    fn operator_skill_generate(&self, skill_id: String, skill_level: usize) -> Result<Skill> {
        let os =
            from_value::<OfficialSkill>(self.skill_table[skill_id]["levels"][skill_level].clone())?;
        let s: Skill = os.into();
        Ok(s)
    }

    pub(super) fn get_operator_key(&self, name: &String) -> Option<String> {
        for (k, v) in self.character_table.as_object().unwrap() {
            let en = v["appellation"].as_str().unwrap();
            let cn = v["name"].as_str().unwrap();
            let nr = name.as_str();
            if nr == en || nr == cn {
                return Some(k.clone());
            }
        }
        None
    }
}

impl OfficialRange {
    pub(super) fn merge(&mut self) -> Vec<GridRect> {
        let mut r = Vec::<GridRect>::new();
        let v = &mut self.grids;
        v.sort_by(|a, b| {
            if a.col != b.col {
                a.col.cmp(&b.col)
            } else {
                a.row.cmp(&b.row)
            }
        });
        let mut i = 0;
        while i < v.len() {
            let s = v[i];
            let mut gr = GridRect { ul: s, dr: s };
            while i + 1 < v.len() && v[i + 1].row == gr.dr.row + 1 && v[i + 1].col == gr.dr.col {
                gr.dr.row += 1;
                i += 1;
            }
            r.push(gr);
            i += 1;
        }

        let mut merged = Vec::<GridRect>::new();
        for gr in r {
            if let Some(last) = merged.last_mut() {
                if last.dr.row == gr.dr.row && last.ul.row == gr.ul.row {
                    if last.dr.col + 1 == gr.ul.col || last.dr.col - 1 == gr.ul.col {
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
