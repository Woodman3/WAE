pub(crate) mod operator_loader;
pub(crate) mod level_loader;

use serde_json::Value;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub(crate) struct Loader{
    character_table:Value,
    range_table:Value,
    gamedata_const:Value,
    skill_table:Value
}

impl Loader{
    pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Loader> {
        let character_table = load_json_file(path.as_ref().join("character_table.json"))?;
        let range_table = load_json_file(path.as_ref().join("range_table.json"))?;
        let gamedata_const = load_json_file(path.as_ref().join("gamedata_const.json"))?;
        let skill_table = load_json_file(path.as_ref().join("skill_table.json"))?;

        Ok(Loader {
            character_table,
            range_table,
            gamedata_const,
            skill_table,
        })
    }
}