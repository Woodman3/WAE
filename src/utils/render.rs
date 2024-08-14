use std::collections::HashMap;

use super::render_config::*;
use crate::{
    frame::Frame,
    map::tile::{DEPLOY_LOW, DEPLOY_NONE, PASS_ALL},
};
use tiny_skia::*;

pub struct Render<'a> {
    frame: &'a Frame,
    pixmap: Pixmap,
}
impl<'a> Render<'a> {
    pub fn new(f: &'a Frame) -> Self {
        let map_width = f.map.width;
        let map_height = f.map.height;
        let figure_width = map_width as u32 * *BLOCK_SIZE as u32 + *PADDING as u32 * 2;
        let figure_height = map_height as u32 * *BLOCK_SIZE as u32 + *PADDING as u32 * 2;
        let pixmap = Pixmap::new(figure_width, figure_height).unwrap();
        Self { frame: f, pixmap }
    }
    fn paint_block(&mut self) {
        let mut block_paint = Paint::default();
        block_paint.set_color(*BLOCK_COLOR);
        if let Some(block_path) = {
            let mut pb = PathBuilder::new();
            for i in 0..=self.frame.map.height {
                pb.move_to(*PADDING as f32, *PADDING as f32 + i as f32 * *BLOCK_SIZE);
                pb.line_to(
                    *PADDING as f32 + self.frame.map.width as f32 * *BLOCK_SIZE,
                    *PADDING as f32 + i as f32 * *BLOCK_SIZE,
                );
            }
            for i in 0..=self.frame.map.width {
                pb.move_to(*PADDING as f32 + i as f32 * *BLOCK_SIZE, *PADDING as f32);
                pb.line_to(
                    *PADDING as f32 + i as f32 * *BLOCK_SIZE,
                    *PADDING as f32 + self.frame.map.height as f32 * *BLOCK_SIZE,
                );
            }
            pb.finish()
        } {
            let block_stroke = Stroke::default();
            self.pixmap.stroke_path(
                &block_path,
                &block_paint,
                &block_stroke,
                Transform::identity(),
                None,
            );
        }
        let mut p: HashMap<(u8, u8, u8), PathBuilder> = HashMap::new();
        for i in 0..self.frame.map.height as usize {
            for j in 0..self.frame.map.width as usize {
                let block = self.frame.map.layout[i][j];
                let mut block_paint = Paint::default();
                let r = match block & PASS_ALL {
                    0 => 255,
                    _ => 0,
                };
                let g = match block & DEPLOY_NONE {
                    0 => 255,
                    _ => 0,
                };
                let b = match block & DEPLOY_LOW {
                    0 => 255,
                    _ => 0,
                };
                let a = 255;
                let x = *PADDING + j as f32 * *BLOCK_SIZE;
                let y = *PADDING + i as f32 * *BLOCK_SIZE;
                block_paint.set_color(Color::from_rgba8(r, g, b, a));
                if let Some(pb) =p.get_mut(&(r,g,b)){
                    pb.push_rect(Rect::from_xywh(x, y, *BLOCK_SIZE, *BLOCK_SIZE).unwrap());
                }else{
                    let mut pb = PathBuilder::new();
                    pb.push_rect(Rect::from_xywh(x, y, *BLOCK_SIZE, *BLOCK_SIZE).unwrap());
                    p.insert((r,g,b),pb);
                }
            }
        }
        for ((r,g,b),path) in p{
            if let Some(path) = path.finish(){
                let mut block_paint = Paint::default();
                block_paint.set_color(Color::from_rgba8(r, g, b, 255));
                self.pixmap.fill_path(
                    &path,
                    &block_paint,
                    FillRule::EvenOdd,
                    Transform::identity(),
                    None,
                );
            }
        }
        // a different way to paint block , doc say it will slower than the above way,but i seem no difference, so i keep it here
        // for i in 0..self.frame.map.height as usize {
        //     for j in 0..self.frame.map.width as usize {
        //         let block = self.frame.map.layout[i][j];
        //         let mut block_paint = Paint::default();
        //         let r = match block & PASS_ALL {
        //             0 => 255,
        //             _ => 0,
        //         };
        //         let g = match block & DEPLOY_NONE {
        //             0 => 255,
        //             _ => 0,
        //         };
        //         let b = match block & DEPLOY_LOW {
        //             0 => 255,
        //             _ => 0,
        //         };
        //         let a = 255;
        //         block_paint.set_color(Color::from_rgba8(r, g, b, a));
        //         let pb = PathBuilder::new();
        //         let x = *PADDING + j as f32 * *BLOCK_SIZE;
        //         let y = *PADDING + i as f32 * *BLOCK_SIZE;
        //         self.pixmap.fill_rect(
        //             Rect::from_xywh(x, y, *BLOCK_SIZE, *BLOCK_SIZE).unwrap(),
        //             &block_paint,
        //             Transform::identity(),
        //             None,
        //         )
        //     }
        // }
    }
    fn paint_enemy(&mut self) {
        let mut enemy_paint = Paint::default();
        enemy_paint.set_color(*ENEMY_COLOR);
        if let Some(enemy_path) = {
            let mut pb = PathBuilder::new();
            for e in &self.frame.enemy_set {
                let (mut x, mut y) = e.borrow().location.into();
                x = *PADDING + x * *BLOCK_SIZE;
                y = *PADDING + y * *BLOCK_SIZE;
                pb.push_circle(x, y, *ENEMY_RADIUS);
            }
            pb.finish()
        } {
            let enemy_stroke = Stroke::default();
            self.pixmap.stroke_path(
                &enemy_path,
                &enemy_paint,
                &enemy_stroke,
                Transform::identity(),
                None,
            );
        }
    }
    fn paint_operator(&mut self) {
        let mut operator_paint = Paint::default();
        operator_paint.set_color(*OPERATOR_COLOR);
        if let Some(operator_path) = {
            let mut pb = PathBuilder::new();
            for (_key, o) in &self.frame.operator_deploy {
                let o = o.borrow();
                let p = super::math::Point::from(o.location);
                let (mut x, mut y) = p.into();
                x = *PADDING + x * *BLOCK_SIZE;
                y = *PADDING + y * *BLOCK_SIZE;
                pb.push_circle(x, y, *OPERATOR_RADIUS);
            }
            pb.finish()
        } {
            let operator_stroke = Stroke::default();
            self.pixmap.stroke_path(
                &operator_path,
                &operator_paint,
                &operator_stroke,
                Transform::identity(),
                None,
            );
        }
    }
    fn paint_bullet(&mut self) {
        let mut bullet_paint = Paint::default();
        bullet_paint.set_color(*BULLET_COLOR);
        if let Some(bullet_path) = {
            let mut pb = PathBuilder::new();
            for b in &self.frame.bullet_set {
                let (mut x, mut y) = b.location.into();
                x = *PADDING + x * *BLOCK_SIZE;
                y = *PADDING + y * *BLOCK_SIZE;
                pb.push_circle(x, y, *BULLET_RADIUS);
            }
            pb.finish()
        } {
            let bullet_stroke = Stroke::default();
            self.pixmap.stroke_path(
                &bullet_path,
                &bullet_paint,
                &bullet_stroke,
                Transform::identity(),
                None,
            );
        }
    }
    pub fn render(&mut self) {
        self.paint_block();
        self.paint_enemy();
        self.paint_operator();
        self.paint_bullet();
    }
    pub fn save(&self) {
        self.pixmap.save_png("image.png").unwrap();
    }
    pub fn encode(&self) -> Vec<u8> {
        self.pixmap.encode_png().unwrap()
    }
}

#[cfg(test)]
mod test {}
