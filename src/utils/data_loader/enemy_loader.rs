use serde::Deserialize;

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
    pub(super) attribute:OfficalEnemyDataTemplate<String>,
}

struct OfficalEnemyAttribute{
    pub(super) maxHp:OfficalEnemyDataTemplate<u32>,
    pub(super) atk:OfficalEnemyDataTemplate<u32>,
    pub(super) def:OfficalEnemyDataTemplate<u32>,
    pub(super) magidResistance:OfficalEnemyDataTemplate<f32>,
    pub(super) cost:OfficalEnemyDataTemplate<u32>,
    pub(super) blockCnt:OfficalEnemyDataTemplate<u32>,
    pub(super) moveSpeed:OfficalEnemyDataTemplate<f32>,
    pub(super) attackSpeed:OfficalEnemyDataTemplate<f32>,
    pub(super) baseAttackTime:OfficalEnemyDataTemplate<f32>,
    pub(super) respawnTime:OfficalEnemyDataTemplate<u32>,
    pub(super) hpRecoveryPerSec:OfficalEnemyDataTemplate<f32>,
    pub(super) spRecoveryPerSec:OfficalEnemyDataTemplate<f32>,
    pub(super) maxDeployCount:OfficalEnemyDataTemplate<u32>,
    pub(super) massLevel:OfficalEnemyDataTemplate<u32>,
    pub(super) baseForceLevel:OfficalEnemyDataTemplate<u32>,
    pub(super) tauntLevel:OfficalEnemyDataTemplate<u32>,
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
    pub(super) m_value:T,
}

#[cfg(test)]
mod test{
    use super::*;
    use serde_json::json;
    #[test]
    fn test_offical_enemy(){

    }
}