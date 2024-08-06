use std::ops::Add;

use serde::Deserialize;
use serde_json::from_value;

use crate::unit::{enemy::{self, Enemy}, UnitInfo};
use super::Loader;
use super::Result;

#[derive(Deserialize,Default,Debug,Clone)]
pub(super) struct OfficialEnemy{
    pub(super) Key:String,
    pub(super) Value:Vec<OfficialEnemyValue>
}

#[derive(Deserialize,Default,Debug,Clone)]
pub(super) struct OfficialEnemyValue{
    pub(super) level:i32,
    pub(super) enemyData:OfficialEnemyData, 
}

#[derive(Deserialize,Default,Debug,Clone)]
pub(super) struct OfficialEnemyData{
    pub(super) name:OfficialEnemyDataTemplate<String>,
    pub(super) applyWay:OfficialEnemyDataTemplate<String>,
    pub(super) motion:OfficialEnemyDataTemplate<String>,
    pub(super) lifePointReduce:OfficialEnemyDataTemplate<u64>,
    pub(super) attributes:OfficialEnemyAttribute,
}

#[derive(Deserialize,Default,Debug,Clone)]
pub(super) struct OfficialEnemyAttribute{
    pub(super) maxHp:OfficialEnemyDataTemplate<i64>,
    pub(super) atk:OfficialEnemyDataTemplate<i64>,
    pub(super) def:OfficialEnemyDataTemplate<i64>,
    pub(super) magicResistance:OfficialEnemyDataTemplate<f64>,
    pub(super) cost:OfficialEnemyDataTemplate<i64>,
    pub(super) blockCnt:OfficialEnemyDataTemplate<i64>,
    pub(super) moveSpeed:OfficialEnemyDataTemplate<f64>,
    pub(super) attackSpeed:OfficialEnemyDataTemplate<f64>,
    pub(super) baseAttackTime:OfficialEnemyDataTemplate<f64>,
    pub(super) respawnTime:OfficialEnemyDataTemplate<i64>,
    pub(super) hpRecoveryPerSec:OfficialEnemyDataTemplate<f64>,
    pub(super) spRecoveryPerSec:OfficialEnemyDataTemplate<f64>,
    pub(super) maxDeployCount:OfficialEnemyDataTemplate<i64>,
    pub(super) massLevel:OfficialEnemyDataTemplate<i64>,
    pub(super) baseForceLevel:OfficialEnemyDataTemplate<i64>,
    pub(super) tauntLevel:OfficialEnemyDataTemplate<i64>,
    pub(super) epDamageResistance:OfficialEnemyDataTemplate<f64>,
    pub(super) epResistance:OfficialEnemyDataTemplate<f64>,
    pub(super) damageHitratePhysical:OfficialEnemyDataTemplate<f64>,
    pub(super) damageHitrateMagical:OfficialEnemyDataTemplate<f64>,
    pub(super) stunImmune:OfficialEnemyDataTemplate<bool>,
    pub(super) silenceImmune:OfficialEnemyDataTemplate<bool>,
    pub(super) sleepImmune:OfficialEnemyDataTemplate<bool>,
    pub(super) frozenImmune:OfficialEnemyDataTemplate<bool>,
    pub(super) levitateImmune:OfficialEnemyDataTemplate<bool>,
    pub(super) disarmedCombatImmune:OfficialEnemyDataTemplate<bool>,
}

#[derive(Deserialize,Default,Debug,Clone)]
pub(super) struct OfficialEnemyDataTemplate<T>
{
    pub(super) m_defined:bool,
    pub(super) m_value:Option<T>,
}

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
        let move_speed = att.moveSpeed.m_value.unwrap();
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
    
        let max_hp = self.maxHp.m_value.unwrap();
        let atk = self.atk.m_value.unwrap();
        let def = self.def.m_value.unwrap();
        let magic_resist = self.magicResistance.m_value.unwrap();
        let attack_time = self.baseAttackTime.m_value.unwrap();
        let block_num = self.blockCnt.m_value.unwrap();
        let aspd = self.attackSpeed.m_value.unwrap();
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
        Ok(enemy.enemyData.clone().into())
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