pub mod config;
pub(super) mod copilot;
pub(super) mod data_loader;
pub mod debugger;
pub mod error;
pub mod math;
pub(super) mod render;
mod render_config;

use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn get_short_type_name<'a, T>() -> &'a str {
    let tn = std::any::type_name::<T>();

    if let Some(index) = tn.rfind(":") {
        let slice = &tn[index + 1..];
        return slice;
    }
    tn
}

fn load_json_file<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> Result<T> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let r = serde_json::from_reader(reader)?;
    Ok(r)
}
