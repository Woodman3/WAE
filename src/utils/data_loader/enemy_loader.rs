use std::ops::Add;

use serde::Deserialize;
use serde_json::from_value;

use crate::unit::{enemy::{self, Enemy}, UnitInfo};
use super::Loader;
use super::Result;

#[derive(Deserialize,Default,Debug,Clone)]
#[serde(rename_all = "PascalCase")]
pub(super) struct OfficialEnemy{
    pub(super) key:String,
    pub(super) value:Vec<OfficialEnemyValue>
}

#[derive(Deserialize,Default,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialEnemyValue{
    pub(super) level:i32,
    pub(super) enemy_data:OfficialEnemyData, 
}

#[derive(Deserialize,Default,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialEnemyData{
    pub(super) name:OfficialEnemyDataTemplate<String>,
    pub(super) apply_way:OfficialEnemyDataTemplate<String>,
    pub(super) motion:OfficialEnemyDataTemplate<String>,
    pub(super) life_point_reduce:OfficialEnemyDataTemplate<u64>,
    pub(super) attributes:OfficialEnemyAttribute,
}

#[derive(Deserialize,Default,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct OfficialEnemyAttribute{
    pub(super) max_hp:OfficialEnemyDataTemplate<i64>,
    pub(super) atk:OfficialEnemyDataTemplate<i64>,
    pub(super) def:OfficialEnemyDataTemplate<i64>,
    pub(super) magic_resistance:OfficialEnemyDataTemplate<f64>,
    pub(super) cost:OfficialEnemyDataTemplate<i64>,
    pub(super) block_cnt:OfficialEnemyDataTemplate<i64>,
    pub(super) move_speed:OfficialEnemyDataTemplate<f64>,
    pub(super) attack_speed:OfficialEnemyDataTemplate<f64>,
    pub(super) base_attack_time:OfficialEnemyDataTemplate<f64>,
    pub(super) respawn_time:OfficialEnemyDataTemplate<i64>,
    pub(super) hp_recovery_per_sec:OfficialEnemyDataTemplate<f64>,
    pub(super) sp_recovery_per_sec:OfficialEnemyDataTemplate<f64>,
    pub(super) max_deploy_count:OfficialEnemyDataTemplate<i64>,
    pub(super) mass_level:OfficialEnemyDataTemplate<i64>,
    pub(super) base_force_level:OfficialEnemyDataTemplate<i64>,
    pub(super) taunt_level:OfficialEnemyDataTemplate<i64>,
    pub(super) ep_damage_resistance:OfficialEnemyDataTemplate<f64>,
    pub(super) ep_resistance:OfficialEnemyDataTemplate<f64>,
    pub(super) damage_hitrate_physical:OfficialEnemyDataTemplate<f64>,
    pub(super) damage_hitrate_magical:OfficialEnemyDataTemplate<f64>,
    pub(super) stun_immune:OfficialEnemyDataTemplate<bool>,
    pub(super) silence_immune:OfficialEnemyDataTemplate<bool>,
    pub(super) sleep_immune:OfficialEnemyDataTemplate<bool>,
    pub(super) frozen_immune:OfficialEnemyDataTemplate<bool>,
    pub(super) levitate_immune:OfficialEnemyDataTemplate<bool>,
    pub(super) disarmed_combat_immune:OfficialEnemyDataTemplate<bool>,
}

#[derive(Deserialize,Default,Debug,Clone)]
pub(super) struct OfficialEnemyDataTemplate<T>
{
    pub(super) m_defined:bool,
    pub(super) m_value:Option<T>,
}

// impl Add for OfficialEnemyAttribute{
//     type Output = Self;
//     fn add(self,other:Self) -> Self{
//         let max_hp = self.maxHp+other.maxHp;
//         let atk = self.atk+other.atk;
//         let def = self.def+other.def;
//         let magic_resist = self.magicResistance+other.magicResistance;
//         let attack_time = self.baseAttackTime+other.baseAttackTime;
//         let block_num = self.blockCnt+other.blockCnt;
//         let aspd = self.attackSpeed+other.attackSpeed;

//         Self{
//             maxHp:max_hp,
//             atk,
//             def,
//             magicResistance:magic_resist,
//             baseAttackTime:attack_time,
//             blockCnt:block_num,
//             attackSpeed:aspd,
//             ..Default::default()
//         }
//     }
// }

/// right value will overwrite left value if it is defined 
impl<T> Add for OfficialEnemyDataTemplate<T>{
    type Output = Self;
    fn add(self,other:Self) -> Self{
        let m_defined = self.m_defined || other.m_defined;
        if other.m_value.is_none(){
            return Self{
                m_defined,
                m_value:self.m_value,
            }
        }else{
            return Self{
                m_defined,
                m_value:other.m_value,
            }
        }
    }
}

impl Into<Enemy> for OfficialEnemyData {
    fn into(self) -> Enemy {
        let name = self.name.m_value.unwrap();
        let att = self.attributes;
        let move_speed = att.move_speed.m_value.unwrap();
        let info:UnitInfo = att.into(); 
        let stage = info.clone();
        Enemy{
            name,
            move_speed,
            info,
            stage,
            ..Default::default()
        }
    }
}

impl Into<UnitInfo> for OfficialEnemyAttribute{
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
        UnitInfo{
            damage_type,
            hp:max_hp,
            max_hp,
            aspd,
            atk,
            def,
            magic_resist,
            attack_time,
            block_num,
            attack_type,
        }
    
    }
}

impl Loader{
    pub(crate) fn load_official_enemy(&self,key:&String,level:usize) -> Result<Enemy> {
        let data = self.enemy_database.get(key).ok_or("Key not found")?;
        let enemy = data.get(level).ok_or("Level not found")?;
        Ok(enemy.enemy_data.clone().into())
    }
    
}

#[cfg(test)]
mod test{

    #[test]
    fn test_template(){
        let a = super::OfficialEnemyDataTemplate{
            m_defined:true,
            m_value:Some(1),
        };
        let b = super::OfficialEnemyDataTemplate{
            m_defined:true,
            m_value:Some(2),
        };
        let c =a+b;
        assert_eq!(c.m_value,Some(2));
    }

}