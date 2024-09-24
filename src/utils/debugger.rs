use super::render::Render;
use crate::calculator::Calculator;
use crate::frame::Frame;
use eframe::egui;
use eframe::egui::{Context, TextFormat, Ui};
use egui_extras::install_image_loaders;

use log::{Level, Metadata, Record};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
pub(crate) struct Debugger {
    pub(crate) c: Calculator,
    pub(crate) run: bool,
    pub(crate) paint: bool,
    pub(crate) log_receiver: Arc<Mutex<Receiver<String>>>,
    pub(crate) log_messages: Arc<Mutex<Vec<String>>>,
}

pub(crate) struct DebugLogger {
    pub(crate) sender: Sender<String>,
}

impl log::Log for DebugLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let _ = self
                .sender
                .send(format!("{} - {}", record.level(), record.args()));
        }
    }

    fn flush(&self) {}
}

impl Debugger {
    pub(crate) fn new(cc: &eframe::CreationContext<'_>,c: Calculator, receiver: Receiver<String>) -> Self {
        Self::setup_custom_fonts(&cc.egui_ctx);
        Self {
            c,
            run: false,
            paint: false,
            log_receiver: Arc::new(Mutex::new(receiver)),
            log_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn paint_info(&self, f: &Frame, ui: &mut Ui) {
        // let text=RichText("a text".into());
        let mut info = egui::text::LayoutJob::default();
        let text_format = TextFormat::default();
        let text = format!("{}\n", f);
        info.append(text.as_str(), 0.0, text_format.clone());
        // for e in f.enemy_set.iter() {
        //     let e = e.borrow();
        //     let text = format!("{e}\n");
        //     info.append(text.as_str(), 0.0, text_format.clone());
        // }
        // for (name, o) in f.operator_deploy.iter() {
        //     let o = o.borrow();
        //     let text = format!("{name},{o}\n");
        //     info.append(text.as_str(), 0.0, text_format.clone());
        // }
        // for b in f.bullet_set.iter() {
        //     let text = format!("{b}");
        //     info.append(text.as_str(), 0.0, text_format.clone());
        // }
        ui.label(info);
    }
    fn paint_log(&self, ui: &mut Ui) {
        let receiver = self.log_receiver.lock().unwrap();
        while let Ok(message) = receiver.try_recv() {
            self.log_messages.lock().unwrap().push(message);
        }
        for message in self.log_messages.lock().unwrap().iter() {
            ui.label(message);
        }
    }
    fn paint_frame(&self, ctx: &Context, ui: &mut Ui) {
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

    fn setup_custom_fonts(ctx: &egui::Context) {
        // Start with the default fonts (we will be adding to them rather than replacing them).
        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        fonts.font_data.insert(
            "MiSan".to_owned(),
            egui::FontData::from_static(include_bytes!("../../assets/MiSans-Regular.otf")),
        );

        // Put my font first (highest priority) for proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "MiSan".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("MiSan".to_owned());

        // Tell egui to use these fonts:
        ctx.set_fonts(fonts);
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
                ui.checkbox(&mut self.paint, "paint");
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
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.paint {
                self.paint_frame(ctx, ui);
            }
            // self.paint_frame(ctx, ui);
        });
    }
}
