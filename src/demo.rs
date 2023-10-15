use eframe::{egui, epaint, Frame};
use eframe::egui::{Color32, Context, Painter, pos2, Sense, vec2};
use eframe::egui::ImageData::Color;
use eframe::egui::Key::F;
use eframe::egui::Shape::Rect;
use eframe::epaint::RectShape;
use env_logger::fmt::Color::Black;

pub  fn fun(){
    let mut native_config=eframe::NativeOptions::default();
    native_config.initial_window_size=vec2(1000.0,500.0).into();
    let mut f =2.0;
    eframe::run_native("demo",native_config,Box::new(|cc| Box::new(App::new(cc,2.0))));
}

pub struct App{
    v: f32
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>,v: f32) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self{
            v
        }
    }
}

impl eframe::App for App  {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::Context::request_repaint(ctx);
        self.v+=0.1;
        egui::CentralPanel::default().show(ctx,|ui|{
            // 添加一个空Shape
            let painter=Painter::new(
                ctx.clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            );
            let shape = painter.add(egui::Shape::Noop);
            // 定义圆角大小
            let rounding = egui::Rounding::same(4.0);
            // 定义要绘制的矩形大小及位置
            let body_rect = egui::Rect::from_min_size(egui::pos2(self.v, 100.0), egui::vec2(100.0, 200.0));
            // 添加一个矩形
            let body = egui::Shape::Rect(RectShape {
                rect: body_rect,
                rounding: rounding,
                fill: Color32::BLUE,
                stroke: egui::Stroke::NONE,
                fill_texture_id: Default::default(),
                uv: epaint::Rect::ZERO,
            });
            // 绘制
            // painter.set(shape, body.clone());
            ui.painter().add(body.clone());
        });
    }

}