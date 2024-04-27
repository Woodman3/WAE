pub(crate) mod operator_loader;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
struct Loader{
    character_table:Value,
    range_table:Value,
    gamedata_const:Value,
    skill_table:Value
}

