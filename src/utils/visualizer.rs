use super::render::Render;
use crate::calculator::Calculator;
use crate::frame::Frame;
use eframe::egui;
use eframe::egui::{Context, Painter, TextFormat, Ui};
use egui_extras::install_image_loaders;

pub struct Visualizer {
    pub c: Calculator,
    run: bool,
}

fn paint_info(f: &Frame, ui: &mut Ui) {
    // let text=RichText("a text".into());
    let mut info = egui::text::LayoutJob::default();
    let text_format = TextFormat::default();
    let text = format!("time_stamp:{}\n", f.timestamp);
    info.append(text.as_str(), 0.0, text_format.clone());
    for e in f.enemy_set.iter() {
        let e = e.borrow();
        let text = format!("{e}\n");
        info.append(text.as_str(), 0.0, text_format.clone());
    }
    for (name, o) in f.operator_deploy.iter() {
        let o = o.borrow();
        let text = format!("{name},{o}\n");
        info.append(text.as_str(), 0.0, text_format.clone());
    }
    for b in f.bullet_set.iter() {
        let text = format!("{b}");
        info.append(text.as_str(), 0.0, text_format.clone());
    }
    ui.label(info);
}
impl Visualizer {
    pub fn new(_cc: &eframe::CreationContext<'_>, c: Calculator) -> Self {
        Self { c, run: false }
    }
}
impl eframe::App for Visualizer {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::Context::request_repaint(ctx);
        install_image_loaders(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut r = Render::new(&self.c.frame_vec[0]);
            r.render();
            let image_bytes = r.encode();
            let img =
                image::load_from_memory_with_format(&image_bytes, image::ImageFormat::Png).unwrap();
            let img = img.as_rgba8().unwrap();
            let (width, height) = img.dimensions();
            let image_data =
                egui::ColorImage::from_rgba_unmultiplied([width as usize, height as usize], &img);
            let texture = ctx.load_texture("frame", image_data, egui::TextureOptions::LINEAR);
            ui.image((texture.id(), texture.size_vec2()));
        });
        egui::SidePanel::right("debug")
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.checkbox(&mut self.run, "run");
                if ui.button("next").clicked() || self.run {
                    self.c.step();
                };
                if ui.button("save_frame").clicked() {
                    if let Some(f) = self.c.get_frame() {
                        let j = serde_json::to_string_pretty(f).unwrap();
                        std::fs::write("frame.json", j).unwrap();
                    }
                }
                paint_info(&self.c.frame_vec[0], ui);
            });
    }
}
