use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;
use std::sync::Mutex;
use tiny_skia::Color;

#[derive(Deserialize)]
struct Config {
    operator_color: [u8; 4],
    bullet_color: [u8; 4],
    enemy_color: [u8; 4],
    enemy_radius: f32,
    operator_radius: f32,
    bullet_radius: f32,
    block_size: f32,
    padding: f32,
    block_color: [u8; 4],
}

lazy_static! {
    static ref CONFIG: Mutex<Config> = {
        let config_str = fs::read_to_string("config/visualizer_config.json")
            .expect("Failed to read config file");
        let config: Config =
            serde_json::from_str(&config_str).expect("Failed to parse config file");
        Mutex::new(config)
    };
    pub static ref OPERATOR_COLOR: Color = {
        let config = CONFIG.lock().unwrap();
        Color::from_rgba8(
            config.operator_color[0],
            config.operator_color[1],
            config.operator_color[2],
            config.operator_color[3],
        )
    };
    pub static ref BULLET_COLOR: Color = {
        let config = CONFIG.lock().unwrap();
        Color::from_rgba8(
            config.bullet_color[0],
            config.bullet_color[1],
            config.bullet_color[2],
            config.bullet_color[3],
        )
    };
    pub static ref ENEMY_COLOR: Color = {
        let config = CONFIG.lock().unwrap();
        Color::from_rgba8(
            config.enemy_color[0],
            config.enemy_color[1],
            config.enemy_color[2],
            config.enemy_color[3],
        )
    };
    pub static ref ENEMY_RADIUS: f32 = CONFIG.lock().unwrap().enemy_radius;
    pub static ref OPERATOR_RADIUS: f32 = CONFIG.lock().unwrap().operator_radius;
    pub static ref BULLET_RADIUS: f32 = CONFIG.lock().unwrap().bullet_radius;
    pub static ref BLOCK_SIZE: f32 = CONFIG.lock().unwrap().block_size;
    pub static ref PADDING: f32 = CONFIG.lock().unwrap().padding;
    pub static ref BLOCK_COLOR: Color = {
        let config = CONFIG.lock().unwrap();
        Color::from_rgba8(
            config.block_color[0],
            config.block_color[1],
            config.block_color[2],
            config.block_color[3],
        )
    };
}
