pub(crate) mod operator_loader;
pub(crate) mod level_loader;
pub(crate) mod enemy_loader;

use std::{collections::HashMap, path::{Path, PathBuf}};
use serde_json::{from_value, Value};
use crate::unit::enemy;
use super::load_json_file;
use enemy_loader::{OfficalEnemy,OfficalEnemyValue};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub(crate) struct Loader{
    path:PathBuf,
    character_table:Value,
    range_table:Value,
    gamedata_const:Value,
    skill_table:Value,
    enemy_database:HashMap<String,Vec<OfficalEnemyValue>>,
}

impl Loader{
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Loader> {
        let character_table = load_json_file(path.as_ref().join("character_table.json"))?;
        let range_table = load_json_file(path.as_ref().join("range_table.json"))?;
        let gamedata_const = load_json_file(path.as_ref().join("gamedata_const.json"))?;
        let skill_table = load_json_file(path.as_ref().join("skill_table.json"))?;
        let mut enemy_database_j = load_json_file(path.as_ref().join("levels/enemydata/enemy_database.json"))?;
        enemy_database_j = std::mem::take(&mut enemy_database_j["enemies"]); 
        let enemy_database_v =from_value::<Vec<OfficalEnemy>>(enemy_database_j)?;
        let enemy_database = enemy_database_v.into_iter().map(|enemy| (enemy.Key,enemy.Value)).collect();
        let path = path.as_ref().to_path_buf();
        Ok(Loader {
            path,
            character_table,
            range_table,
            gamedata_const,
            skill_table,
            enemy_database,
        })
    }
}
