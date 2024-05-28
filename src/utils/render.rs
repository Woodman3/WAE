use crate::frame::Frame;
use super::visualizer_config::*;
use tiny_skia::*;

struct Render<'a>{
    frame:&'a Frame,
    pixmap:Pixmap
}
impl<'a> Render<'a> {
    fn new(f: &'a Frame) -> Self {
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
        let block_path = {
            let mut pb = PathBuilder::new();
            for i in 0..=self.frame.map.height {
                pb.move_to(PADING as f32, PADING as f32 + i as f32 * BLOCK_SIZE);
                pb.line_to(PADING as f32 + self.frame.map.width as f32 * BLOCK_SIZE, PADING as f32 + i as f32 * BLOCK_SIZE);
            }
            for i in 0..=self.frame.map.width {
                pb.move_to(PADING as f32 + i as f32 * BLOCK_SIZE, PADING as f32);
                pb.line_to(PADING as f32 + i as f32 * BLOCK_SIZE, PADING as f32 + self.frame.map.height as f32 * BLOCK_SIZE);
            }
            pb.finish().unwrap()
        };
        let block_stroke = Stroke::default();
        self.pixmap.stroke_path(&block_path, &block_paint, &block_stroke, Transform::identity(), None);   
    }
    fn paint_enemy(&mut self){
        let mut enemy_paint = Paint::default();
        enemy_paint.set_color(ENEMY_COLOR);
        let enemy_path = {
            let mut pb = PathBuilder::new();
            for e in &self.frame.enemy_set{
                let (x,y) = e.borrow().location.into();
                x = PADING + x * BLOCK_SIZE;
                y = PADING + y * BLOCK_SIZE;
                pb.push_circle(x, y, ENEMY_RADIUS);
            }
            pb.finish().unwrap()
        };
        let enemy_stroke = Stroke::default();
        self.pixmap.stroke_path(&enemy_path, &enemy_paint, &enemy_stroke, Transform::identity(), None);
    }
    pub(super) fn render(&mut self){
        self.paint_block();
        self.paint_enemy();
    }
    fn save(&self){
        self.pixmap.save_png("image.png").unwrap();
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
    }
}