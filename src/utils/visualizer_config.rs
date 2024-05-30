use crate::map::ENEMY_TOUCH_SIZE;
use eframe::egui::Color32;
use tiny_skia::Color;
use lazy_static::lazy_static;


pub const ENEMY_RADIUS:f32=ENEMY_TOUCH_SIZE as f32 * BLOCK_SIZE;
// pub const ENEMY_COLOR:Color32=Color32::RED;
lazy_static!(
    pub(super) static ref OPERATOR_COLOR:Color=Color::from_rgba8(0, 0, 255, 200);
    pub static ref BULLET_COLOR:Color=Color::from_rgba8(0, 0, 0, 255);
    pub(super) static ref ENEMY_COLOR:Color=Color::from_rgba8(50, 127, 150, 200);
);

pub const OPERATOR_RADIUS:f32=2.0;

// pub const OPERATOR_COLOR:Color32=Color32::BLUE;
pub const BULLET_RADIUS:f32=1.0;
// pub const BULLET_COLOR:Color32=Color32::BLACK;

pub const BLOCK_SIZE:f32=50.0;
pub(super) const PADING:f32=10.0;
pub(super) const BLOCK_COLOR:Color=Color::BLACK;
