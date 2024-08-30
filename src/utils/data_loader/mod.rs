pub(crate) mod enemy_loader;
pub(crate) mod level_loader;
pub(crate) mod operator_loader;
#[cfg(test)]
mod test;

use super::load_json_file;
use enemy_loader::{OfficialEnemy, OfficialEnemyValue};
use serde_json::{from_value, Value};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Default)]
pub(crate) struct Loader {
    path: PathBuf,
    character_table: Value,
    range_table: Value,
    gamedata_const: Value,
    skill_table: Value,
    enemy_database: HashMap<String, Vec<OfficialEnemyValue>>,
}

impl Loader {
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Loader> {
        let path = path.as_ref().join("zh_CN/gamedata");
        let character_table = load_json_file(path.join("excel/character_table.json"))?;
        let range_table = load_json_file(path.join("excel/range_table.json"))?;
        let gamedata_const = load_json_file(path.join("excel/gamedata_const.json"))?;
        let skill_table = load_json_file(path.join("excel/skill_table.json"))?;
        let mut enemy_database_j =
            load_json_file(path.join("levels/enemydata/enemy_database.json"))?;
        enemy_database_j = std::mem::take(&mut enemy_database_j["enemies"]);
        let enemy_database_v = from_value::<Vec<OfficialEnemy>>(enemy_database_j)?;
        let enemy_database = enemy_database_v
            .into_iter()
            .map(|enemy| (enemy.key, enemy.value))
            .collect();
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
