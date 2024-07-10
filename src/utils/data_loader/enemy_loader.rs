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
    pub(crate) fn load_offical_enemy(&self,key:String,level:usize) -> Result<Enemy> {
        if let Some(data) = self.enemy_database.get(&key){
            if let Some(enemy) = self.enemy_database[&key].get(level){
                return Ok(enemy.enemyData.clone().into())
            }
        }
        Err("Key not found".into())
        // let mut data = self.enemy_database["enemies"].clone();
        // let enemies = from_value::<Vec<OfficalEnemy>>(data).unwrap();
        // enemies.into_iter().map(|enemy| enemy.Value.into_iter().map(|value| value.enemyData.into()).collect::<Vec<Enemy>>()).flatten().collect()
    }
    
}

#[cfg(test)]
mod test{
    use crate::utils::load_json_file;
    use super::*;
    use serde_json::{Value,from_value};
    #[test]
    fn test_offical_enemy(){
        let path = "data/levels/enemydata/enemy_database.json";
        let enemy_index = 0;
        let mut data = load_json_file(path).unwrap();
        data = data["enemies"].clone();
        let enemies = from_value::<Vec<OfficalEnemy>>(data).unwrap();
        let enemy = &enemies[enemy_index];
        println!("{:?}",enemy);
    }
}