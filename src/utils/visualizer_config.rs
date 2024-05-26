use crate::map::ENEMY_TOUCH_SIZE;
use eframe::egui::Color32;
use tiny_skia::Color;

pub const ENEMY_RADIUS:f32=ENEMY_TOUCH_SIZE as f32 * BLOCK_SIZE;
pub const ENEMY_COLOR:Color32=Color32::RED;
pub const OPERATOR_RADIUS:f32=2.0;

pub const OPERATOR_COLOR:Color32=Color32::BLUE;
pub const BULLET_RADIUS:f32=1.0;
pub const BULLET_COLOR:Color32=Color32::BLACK;

pub const BLOCK_SIZE:f32=50.0;
pub(super) const PADING:f32=10.0;
pub(super) const BLOCK_COLOR:Color=Color::BLACK;
