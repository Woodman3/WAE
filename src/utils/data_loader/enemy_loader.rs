use std::ops::Add;

use serde::Deserialize;

use super::Loader;
use super::Result;
use crate::unit::scope::Scope;
use crate::unit::{enemy::Enemy, UnitInfo};
use crate::utils::math::Point;

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub(super) struct OfficialEnemy {
    pub(super) key: String,
    pub(super) value: Vec<OfficialEnemyValue>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialEnemyValue {
    pub(super) level: i32,
    pub(super) enemy_data: OfficialEnemyData,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialEnemyData {
    pub(super) name: OfficialEnemyDataTemplate<String>,
    pub(super) apply_way: OfficialEnemyDataTemplate<String>,
    pub(super) motion: OfficialEnemyDataTemplate<String>,
    pub(super) life_point_reduce: OfficialEnemyDataTemplate<u64>,
    pub(super) attributes: OfficialEnemyAttribute,
    /// attack range radius if enemy is melee the m_define will set to false
    pub(super) range_radius: OfficialEnemyDataTemplate<f64>,
}

#[derive(Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialEnemyAttribute {
    pub(super) max_hp: OfficialEnemyDataTemplate<i64>,
    pub(super) atk: OfficialEnemyDataTemplate<i64>,
    pub(super) def: OfficialEnemyDataTemplate<i64>,
    pub(super) magic_resistance: OfficialEnemyDataTemplate<f64>,
    pub(super) cost: OfficialEnemyDataTemplate<i64>,
    pub(super) block_cnt: OfficialEnemyDataTemplate<i64>,
    pub(super) move_speed: OfficialEnemyDataTemplate<f64>,
    pub(super) attack_speed: OfficialEnemyDataTemplate<f64>,
    pub(super) base_attack_time: OfficialEnemyDataTemplate<f64>,
    pub(super) respawn_time: OfficialEnemyDataTemplate<i64>,
    pub(super) hp_recovery_per_sec: OfficialEnemyDataTemplate<f64>,
    pub(super) sp_recovery_per_sec: OfficialEnemyDataTemplate<f64>,
    pub(super) max_deploy_count: OfficialEnemyDataTemplate<i64>,
    pub(super) mass_level: OfficialEnemyDataTemplate<i64>,
    pub(super) base_force_level: OfficialEnemyDataTemplate<i64>,
    pub(super) taunt_level: OfficialEnemyDataTemplate<i64>,
    pub(super) ep_damage_resistance: OfficialEnemyDataTemplate<f64>,
    pub(super) ep_resistance: OfficialEnemyDataTemplate<f64>,
    pub(super) damage_hitrate_physical: OfficialEnemyDataTemplate<f64>,
    pub(super) damage_hitrate_magical: OfficialEnemyDataTemplate<f64>,
    pub(super) stun_immune: OfficialEnemyDataTemplate<bool>,
    pub(super) silence_immune: OfficialEnemyDataTemplate<bool>,
    pub(super) sleep_immune: OfficialEnemyDataTemplate<bool>,
    pub(super) frozen_immune: OfficialEnemyDataTemplate<bool>,
    pub(super) levitate_immune: OfficialEnemyDataTemplate<bool>,
    pub(super) disarmed_combat_immune: OfficialEnemyDataTemplate<bool>,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub(super) struct OfficialEnemyDataTemplate<T> {
    pub(super) m_defined: bool,
    pub(super) m_value: Option<T>,
}

impl Add for OfficialEnemyData {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            name: self.name + other.name,
            apply_way: self.apply_way + other.apply_way,
            motion: self.motion + other.motion,
            life_point_reduce: self.life_point_reduce + other.life_point_reduce,
            attributes: self.attributes + other.attributes,
            range_radius: self.range_radius + other.range_radius,
        }
    }
}

impl Add for OfficialEnemyAttribute {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            max_hp: self.max_hp + other.max_hp,
            atk: self.atk + other.atk,
            def: self.def + other.def,
            magic_resistance: self.magic_resistance + other.magic_resistance,
            cost: self.cost + other.cost,
            block_cnt: self.block_cnt + other.block_cnt,
            move_speed: self.move_speed + other.move_speed,
            attack_speed: self.attack_speed + other.attack_speed,
            base_attack_time: self.base_attack_time + other.base_attack_time,
            respawn_time: self.respawn_time + other.respawn_time,
            hp_recovery_per_sec: self.hp_recovery_per_sec + other.hp_recovery_per_sec,
            sp_recovery_per_sec: self.sp_recovery_per_sec + other.sp_recovery_per_sec,
            max_deploy_count: self.max_deploy_count + other.max_deploy_count,
            mass_level: self.mass_level + other.mass_level,
            base_force_level: self.base_force_level + other.base_force_level,
            taunt_level: self.taunt_level + other.taunt_level,
            ep_damage_resistance: self.ep_damage_resistance + other.ep_damage_resistance,
            ep_resistance: self.ep_resistance + other.ep_resistance,
            damage_hitrate_physical: self.damage_hitrate_physical + other.damage_hitrate_physical,
            damage_hitrate_magical: self.damage_hitrate_magical + other.damage_hitrate_magical,
            stun_immune: self.stun_immune + other.stun_immune,
            silence_immune: self.silence_immune + other.silence_immune,
            sleep_immune: self.sleep_immune + other.sleep_immune,
            frozen_immune: self.frozen_immune + other.frozen_immune,
            levitate_immune: self.levitate_immune + other.levitate_immune,
            disarmed_combat_immune: self.disarmed_combat_immune + other.disarmed_combat_immune,
        }
    }
}

/// right value will overwrite left value if it is defined
impl<T> Add for OfficialEnemyDataTemplate<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let m_defined = self.m_defined || other.m_defined;
        if other.m_value.is_none() {
            return Self {
                m_defined,
                m_value: self.m_value,
            };
        } else {
            return Self {
                m_defined,
                m_value: other.m_value,
            };
        }
    }
}

impl Into<Enemy> for OfficialEnemyData {
    fn into(self) -> Enemy {
        let name = self.name.m_value.unwrap();
        let att = self.attributes;
        let move_speed = att.move_speed.m_value.unwrap();
        let mut info: UnitInfo = att.into();
        let stage = info.clone();
        if self.range_radius.m_defined {
            info.scope = Scope::Circle(Point::default(), self.range_radius.m_value.unwrap());
        } else {
            info.scope = Scope::None;
        }
        Enemy {
            name,
            move_speed,
            info,
            stage,
            ..Default::default()
        }
    }
}

impl Into<UnitInfo> for OfficialEnemyAttribute {
    fn into(self) -> UnitInfo {
        use crate::unit::skill::effect::DamageType;
        use crate::unit::skill::skill_type::AttackType;

        let max_hp = self.max_hp.m_value.unwrap();
        let atk = self.atk.m_value.unwrap();
        let def = self.def.m_value.unwrap();
        let magic_resist = self.magic_resistance.m_value.unwrap();
        let attack_time = self.base_attack_time.m_value.unwrap();
        let block_num = self.block_cnt.m_value.unwrap();
        let aspd = self.attack_speed.m_value.unwrap();
        let damage_type = DamageType::None;
        let attack_type = AttackType::None;
        UnitInfo {
            damage_type,
            hp: max_hp,
            max_hp,
            aspd,
            atk,
            def,
            magic_resist,
            attack_time,
            block_num,
            attack_type,
            ..Default::default()
        }
    }
}

impl Loader {
    pub(crate) fn load_enemy(&self, key: &String, level: usize) -> Result<Enemy> {
        let data = self.enemy_database.get(key).ok_or("Key not found")?;
        let mut enemy = OfficialEnemyData::default();
        for i in 0..=level {
            let rhs = data.get(i).ok_or("Level not found")?;
            enemy = enemy + rhs.enemy_data.clone();
        }
        Ok(enemy.into())
    }
}
