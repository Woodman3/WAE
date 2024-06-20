use serde::Deserialize;

use crate::unit::enemy::Enemy;

#[derive(Deserialize,Default,Debug)]
struct OfficalEnemy{
    pub(super) Key:String,
    pub(super) Value:Vec<OfficalEnemyValue>
}

#[derive(Deserialize,Default,Debug)]
struct OfficalEnemyValue{
    pub(super) level:i32,
    pub(super) enemyData:OfficalEnemyData, 
}

#[derive(Deserialize,Default,Debug)]
struct OfficalEnemyData{
    pub(super) name:OfficalEnemyDataTemplate<String>,
    pub(super) applyWay:OfficalEnemyDataTemplate<String>,
    pub(super) motion:OfficalEnemyDataTemplate<String>,
    pub(super) lifePointReduce:OfficalEnemyDataTemplate<u32>,
    pub(super) attributes:OfficalEnemyAttribute,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalEnemyAttribute{
    pub(super) maxHp:OfficalEnemyDataTemplate<i32>,
    pub(super) atk:OfficalEnemyDataTemplate<i32>,
    pub(super) def:OfficalEnemyDataTemplate<i32>,
    pub(super) magicResistance:OfficalEnemyDataTemplate<f32>,
    pub(super) cost:OfficalEnemyDataTemplate<i32>,
    pub(super) blockCnt:OfficalEnemyDataTemplate<i32>,
    pub(super) moveSpeed:OfficalEnemyDataTemplate<f32>,
    pub(super) attackSpeed:OfficalEnemyDataTemplate<f32>,
    pub(super) baseAttackTime:OfficalEnemyDataTemplate<f32>,
    pub(super) respawnTime:OfficalEnemyDataTemplate<i32>,
    pub(super) hpRecoveryPerSec:OfficalEnemyDataTemplate<f32>,
    pub(super) spRecoveryPerSec:OfficalEnemyDataTemplate<f32>,
    pub(super) maxDeployCount:OfficalEnemyDataTemplate<i32>,
    pub(super) massLevel:OfficalEnemyDataTemplate<i32>,
    pub(super) baseForceLevel:OfficalEnemyDataTemplate<i32>,
    pub(super) tauntLevel:OfficalEnemyDataTemplate<i32>,
    pub(super) epDamageResistance:OfficalEnemyDataTemplate<f32>,
    pub(super) epResistance:OfficalEnemyDataTemplate<f32>,
    pub(super) damageHitratePhysical:OfficalEnemyDataTemplate<f32>,
    pub(super) damageHitrateMagical:OfficalEnemyDataTemplate<f32>,
    pub(super) stunImmune:OfficalEnemyDataTemplate<bool>,
    pub(super) silenceImmune:OfficalEnemyDataTemplate<bool>,
    pub(super) sleepImmune:OfficalEnemyDataTemplate<bool>,
    pub(super) frozenImmune:OfficalEnemyDataTemplate<bool>,
    pub(super) levitateImmune:OfficalEnemyDataTemplate<bool>,
    pub(super) disarmedCombatImmune:OfficalEnemyDataTemplate<bool>,
}

#[derive(Deserialize,Default,Debug)]
struct OfficalEnemyDataTemplate<T>
{
    pub(super) m_defined:bool,
    pub(super) m_value:Option<T>,
}

impl Into<Enemy> for OfficalEnemy {
    fn into(self) -> Enemy {

        todo!()
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