use super::render::Render;
use crate::calculator::Calculator;
use crate::frame::Frame;
use eframe::egui;
use eframe::egui::{Context, Painter, TextFormat, Ui};
use egui_extras::install_image_loaders;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, Sender};
use log::{Record, Level, Metadata, LevelFilter};
pub(crate) struct Debugger {
    pub(crate) c: Calculator,
    pub(crate) run: bool,
    pub(crate) log_receiver: Arc<Mutex<Receiver<String>>>,
    pub(crate) log_messages: Arc<Mutex<Vec<String>>>,

}

pub(crate) struct DebugLogger{
    pub(crate) sender:Sender<String>
}

impl log::Log for DebugLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let _ = self.sender.send(format!("{} - {}", record.level(), record.args()));
        }
    }

    fn flush(&self) {}
}


impl Debugger {
    // pub(crate) fn new(_cc: &eframe::CreationContext<'_>, c: Calculator) -> Self {
    //     Self { c, run: false }
    // }
    fn paint_info(&self,f: &Frame, ui: &mut Ui) {
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
    fn paint_log(&self,ui: &mut Ui){
        let receiver = self.log_receiver.lock().unwrap();
        while let Ok(message) = receiver.try_recv() {
            self.log_messages.lock().unwrap().push(message);
        }
        for message in self.log_messages.lock().unwrap().iter() {
            ui.label(message);
        };
    }
    fn paint_frame(&self,ctx:&Context, ui: &mut Ui) {
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
    }
}
impl eframe::App for Debugger {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::Context::request_repaint(ctx);
        install_image_loaders(ctx);
        egui::SidePanel::left("log")
            .min_width(400.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Log Messages");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.paint_log(ui);
                });
            });
        egui::SidePanel::right("debug")
            .min_width(200.0)
            .resizable(true)
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
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.paint_info(&self.c.frame_vec[0], ui);
                });
            });
        egui::CentralPanel::default()
            .show(ctx, |ui| {
            // if(self.c.frame_vec[0].timestamp%100==0){
                self.paint_frame(ctx, ui);
            // }
            // self.paint_frame(ctx, ui);
        });
    }
}
