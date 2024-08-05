use std::ops::Add;

use serde::Deserialize;
use serde_json::from_value;

use crate::unit::{enemy::{self, Enemy}, UnitInfo};
use super::Loader;
use super::Result;

#[derive(Deserialize,Default,Debug,Clone)]
pub(super) struct OfficalEnemy{
    pub(super) Key:String,
    pub(super) Value:Vec<OfficalEnemyValue>
}

#[derive(Deserialize,Default,Debug,Clone)]
pub(super) struct OfficalEnemyValue{
    pub(super) level:i32,
    pub(super) enemyData:OfficalEnemyData, 
}

#[derive(Deserialize,Default,Debug,Clone)]
pub(super) struct OfficalEnemyData{
    pub(super) name:OfficalEnemyDataTemplate<String>,
    pub(super) applyWay:OfficalEnemyDataTemplate<String>,
    pub(super) motion:OfficalEnemyDataTemplate<String>,
    pub(super) lifePointReduce:OfficalEnemyDataTemplate<u64>,
    pub(super) attributes:OfficalEnemyAttribute,
}

#[derive(Deserialize,Default,Debug,Clone)]
pub(super) struct OfficalEnemyAttribute{
    pub(super) maxHp:OfficalEnemyDataTemplate<i64>,
    pub(super) atk:OfficalEnemyDataTemplate<i64>,
    pub(super) def:OfficalEnemyDataTemplate<i64>,
    pub(super) magicResistance:OfficalEnemyDataTemplate<f64>,
    pub(super) cost:OfficalEnemyDataTemplate<i64>,
    pub(super) blockCnt:OfficalEnemyDataTemplate<i64>,
    pub(super) moveSpeed:OfficalEnemyDataTemplate<f64>,
    pub(super) attackSpeed:OfficalEnemyDataTemplate<f64>,
    pub(super) baseAttackTime:OfficalEnemyDataTemplate<f64>,
    pub(super) respawnTime:OfficalEnemyDataTemplate<i64>,
    pub(super) hpRecoveryPerSec:OfficalEnemyDataTemplate<f64>,
    pub(super) spRecoveryPerSec:OfficalEnemyDataTemplate<f64>,
    pub(super) maxDeployCount:OfficalEnemyDataTemplate<i64>,
    pub(super) massLevel:OfficalEnemyDataTemplate<i64>,
    pub(super) baseForceLevel:OfficalEnemyDataTemplate<i64>,
    pub(super) tauntLevel:OfficalEnemyDataTemplate<i64>,
    pub(super) epDamageResistance:OfficalEnemyDataTemplate<f64>,
    pub(super) epResistance:OfficalEnemyDataTemplate<f64>,
    pub(super) damageHitratePhysical:OfficalEnemyDataTemplate<f64>,
    pub(super) damageHitrateMagical:OfficalEnemyDataTemplate<f64>,
    pub(super) stunImmune:OfficalEnemyDataTemplate<bool>,
    pub(super) silenceImmune:OfficalEnemyDataTemplate<bool>,
    pub(super) sleepImmune:OfficalEnemyDataTemplate<bool>,
    pub(super) frozenImmune:OfficalEnemyDataTemplate<bool>,
    pub(super) levitateImmune:OfficalEnemyDataTemplate<bool>,
    pub(super) disarmedCombatImmune:OfficalEnemyDataTemplate<bool>,
}

#[derive(Deserialize,Default,Debug,Clone)]
pub(super) struct OfficalEnemyDataTemplate<T>
{
    pub(super) m_defined:bool,
    pub(super) m_value:Option<T>,
}

/// right value will overwrite left value if it is defined 
impl<T> Add for OfficalEnemyDataTemplate<T>{
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

impl Into<Enemy> for OfficalEnemyData {
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

impl Into<UnitInfo> for OfficalEnemyAttribute{
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
        let a = super::OfficalEnemyDataTemplate{
            m_defined:true,
            m_value:Some(1),
        };
        let b = super::OfficalEnemyDataTemplate{
            m_defined:true,
            m_value:Some(2),
        };
        let c =a+b;
        assert_eq!(c.m_value,Some(2));
    }

}