use crate::frame::Frame;
use super::visualizer_config::*;
use tiny_skia::*;

pub struct Render<'a>{
    frame:&'a Frame,
    pixmap:Pixmap,
}
impl<'a> Render<'a> {
    pub fn new(f: &'a Frame) -> Self {
        let map_width = f.map.width;
        let map_height = f.map.height;
        let figure_width = map_width as u32 * BLOCK_SIZE as u32 + PADING as u32 * 2;
        let figure_height = map_height as u32 * BLOCK_SIZE as u32 + PADING as u32 * 2;
        let mut pixmap = Pixmap::new(figure_width, figure_height).unwrap();
        Self {
            frame: f,
            pixmap,
        }
    }
    fn paint_block(&mut self){
        let mut block_paint = Paint::default();
        block_paint.set_color(BLOCK_COLOR);
        if let Some(block_path) = {
            let mut pb = PathBuilder::new();
            for i in 0..=self.frame.map.height {
                pb.move_to(PADING as f32, PADING as f32 + i as f32 * BLOCK_SIZE);
                pb.line_to(PADING as f32 + self.frame.map.width as f32 * BLOCK_SIZE, PADING as f32 + i as f32 * BLOCK_SIZE);
            }
            for i in 0..=self.frame.map.width {
                pb.move_to(PADING as f32 + i as f32 * BLOCK_SIZE, PADING as f32);
                pb.line_to(PADING as f32 + i as f32 * BLOCK_SIZE, PADING as f32 + self.frame.map.height as f32 * BLOCK_SIZE);
            }
            pb.finish()
        }{
            let block_stroke = Stroke::default();
            self.pixmap.stroke_path(&block_path, &block_paint, &block_stroke, Transform::identity(), None);   
        }
    }
    fn paint_enemy(&mut self){
        let mut enemy_paint = Paint::default();
        enemy_paint.set_color(*ENEMY_COLOR);
        if let Some(enemy_path) = {
            let mut pb = PathBuilder::new();
            for e in &self.frame.enemy_set{
                let (mut x,mut y) = e.borrow().location.into();
                x = PADING + x * BLOCK_SIZE;
                y = PADING + y * BLOCK_SIZE;
                pb.push_circle(x, y, ENEMY_RADIUS);
            }
            pb.finish()
        }{
            let enemy_stroke = Stroke::default();
            self.pixmap.stroke_path(&enemy_path, &enemy_paint, &enemy_stroke, Transform::identity(), None);
        }
    }
    fn paint_operator(&mut self){
        let mut operator_paint = Paint::default();
        operator_paint.set_color(*OPERATOR_COLOR);
        if let Some(operator_path) = {
            let mut pb = PathBuilder::new();
            for (_key,o) in &self.frame.operator_deploy{
                let o = o.borrow();
                let p = super::math::Point::from(o.location);
                let (mut x,mut y) = p.into();
                x = PADING + x * BLOCK_SIZE;
                y = PADING + y * BLOCK_SIZE;
                pb.push_circle(x, y, OPERATOR_RADIUS);
            }
            pb.finish()
        }{
            let operator_stroke = Stroke::default();
            self.pixmap.stroke_path(&operator_path, &operator_paint, &operator_stroke, Transform::identity(), None);
        }
    }
    fn paint_bullet(&mut self){
        let mut bullet_paint = Paint::default();
        bullet_paint.set_color(*BULLET_COLOR);
        if let Some(bullet_path) = {
            let mut pb = PathBuilder::new();
            for b in &self.frame.bullet_set{
                let (mut x,mut y) = b.location.into();
                x = PADING + x * BLOCK_SIZE;
                y = PADING + y * BLOCK_SIZE;
                pb.push_circle(x, y, BULLET_RADIUS);
            }
            pb.finish()
        }{
            let bullet_stroke = Stroke::default();
            self.pixmap.stroke_path(&bullet_path, &bullet_paint, &bullet_stroke, Transform::identity(), None);
        }
    }
    pub fn render(&mut self){
        self.paint_block();
        self.paint_enemy();
        self.paint_operator();
        self.paint_bullet();
    }
    pub fn save(&self){
        self.pixmap.save_png("image.png").unwrap();
    }
    pub fn encode(&self)->Vec<u8>{
        self.pixmap.encode_png().unwrap()
    }
}

#[cfg(test)]
mod test{
    use tiny_skia::*;
    use crate::frame::Frame;
    use super::*; 
    #[test]
    fn test(){
        let j = std::fs::read("frame.json").unwrap();
        let mut f = serde_json::from_slice::<Frame>(&j).unwrap();
        let mut r = Render::new(&f);
        r.render();
        r.save();
    }

}