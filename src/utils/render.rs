use crate::frame::Frame;
use super::visualizer_config::*;
use tiny_skia::*;

fn render(f:&Frame){
    let map_width = f.map.width;
    let map_height = f.map.height;
    let figure_width = map_width as u32 * BLOCK_SIZE  as u32 + PADING as u32 * 2;
    let figure_height = map_height as u32 * BLOCK_SIZE as u32 + PADING as u32 * 2;
    let mut pixmap = Pixmap::new(figure_width, figure_height).unwrap();

    let mut block_paint = Paint::default();
    block_paint.set_color(BLOCK_COLOR);

    let block_path = {
        let mut pb = PathBuilder::new();
        for i in 0..=map_height {
            pb.move_to(PADING as f32, PADING as f32 + i as f32 * BLOCK_SIZE);
            pb.line_to(PADING as f32 + map_width as f32 * BLOCK_SIZE, PADING as f32 + i as f32 * BLOCK_SIZE);
        }
        for i in 0..map_width {
            pb.move_to(PADING as f32 + i as f32 * BLOCK_SIZE, PADING as f32);
            pb.line_to(PADING as f32 + i as f32 * BLOCK_SIZE, PADING as f32 + map_height as f32 * BLOCK_SIZE);
        }
        pb.finish().unwrap()
    };
    let block_stroke = Stroke::default();
    pixmap.stroke_path(&block_path, &block_paint, &block_stroke, Transform::identity(), None);   

    pixmap.save_png("image.png").unwrap();
}

#[cfg(test)]
mod test{
    use tiny_skia::*;
    #[test]
    fn test(){
        use crate::frame::Frame;
        let mut f = Frame::default();
        f.map.width = 7;
        f.map.height = 4; 
        render(f);
    }
}