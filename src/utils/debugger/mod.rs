pub(crate) mod debugger_parser;

use super::load_json_file;
use super::render::Render;
use crate::calculator::Calculator;
use debugger_parser::DebuggerParser;
use eframe::egui;
use eframe::egui::{Context, Ui};
use egui_extras::install_image_loaders;

use log::{error, Level, Metadata, Record};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
pub(crate) struct Debugger {
    pub(crate) c: Calculator,
    pub(crate) run: bool,
    pub(crate) paint: bool,
    pub(crate) log_receiver: Arc<Mutex<Receiver<String>>>,
    pub(crate) log_messages: Arc<Mutex<Vec<String>>>,
    debugger_input: String,
    parser: DebuggerParser,
    config: DebuggerConfig,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct DebuggerConfig {
    init_command: Vec<String>,
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
    pub(crate) fn new(
        cc: &eframe::CreationContext<'_>,
        c: Calculator,
        receiver: Receiver<String>,
        config_path: &Path,
    ) -> Self {
        Self::setup_custom_fonts(&cc.egui_ctx);
        let config: DebuggerConfig = load_json_file(config_path).unwrap_or_default();
        let f = c.frame_vec.last().unwrap();
        let mut parser = DebuggerParser::default();
        for command in config.init_command.iter() {
            if matches!(parser.parse(command, f), Err(_)) {
                warn!("init command error:{}", command);
            }
        }
        Self {
            c,
            run: false,
            paint: false,
            log_receiver: Arc::new(Mutex::new(receiver)),
            log_messages: Arc::new(Mutex::new(Vec::new())),
            // config,
            debugger_input: String::new(),
            parser,
            config,
        }
    }

    // fn paint_info(&mut self, f: &Frame, ui: &mut Ui) {
    //     let config = &mut self.config;
    //     ui.checkbox(&mut config.operator, "operator");
    //     ui.checkbox(&mut config.timer.0, "timer");
    //     ui.checkbox(&mut config.enemy, "enemy");
    //     let text_format = TextFormat::default();
    //     if config.timer.0 {
    //         let config = &mut config.timer.1;
    //         ui.collapsing("timer",|ui|{
    //             ui.checkbox(&mut config.global, "global");
    //             if config.global {
    //                 ui.label(format!("global:{}",f.timer.global));
    //             }
    //             ui.checkbox(&mut config.subwave, "fragment");
    //             if config.subwave {
    //                 ui.label(format!("subwave:{}",f.timer.subwave));
    //             }
    //             ui.checkbox(&mut config.wave, "wave");
    //             if config.wave {
    //                 ui.label(format!("wave:{}",f.timer.wave));
    //             }
    //         });
    //     };

    //     // info.append(text.as_str(), 0.0, text_format.clone());
    //     // ui.label(info);
    // }
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
            egui::FontData::from_static(include_bytes!("../../../assets/MiSans-Regular.otf")),
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

    fn debug_window(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.checkbox(&mut self.run, "run");
        ui.checkbox(&mut self.paint, "paint");
        if ui.button("next").clicked() || self.run {
            self.c.step();
            self.parser.paint_buffer.clear();
        };
        if ui.button("save_frame").clicked() {
            if let Some(f) = self.c.get_frame() {
                let j = serde_json::to_string_pretty(f).unwrap();
                std::fs::write("frame.json", j).unwrap();
            }
        }
        ui.text_edit_singleline(&mut self.debugger_input);
        self.debugger_command(ctx, ui);
        egui::ScrollArea::vertical().show(ui, |ui| {
            // self.paint_info(&f, ui);
            unsafe {
                let message = self.parser.show_pointer();
                ui.label(message);
            }
        });
    }

    fn debugger_command(&mut self, ctx: &Context, ui: &mut Ui) {
        let f = self.c.frame_vec.last().unwrap();
        if ctx.input(|i| 
            i.key_pressed(egui::Key::Enter)) {
            match self.parser.parse(&self.debugger_input, f) {
                Ok(_) => {
                    self.debugger_input.clear();
                    
                }
                Err(e) => {
                    error!("parser error : {:?}", e);
                }
            }
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            // Navigate up in history
            if let Some(last_command) = self.parser.history_up() {
                self.debugger_input = last_command.clone();
            }
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            // Navigate down in history
            if let Some(next_command) = self.parser.history_down() {
                self.debugger_input = next_command.clone();
            }
        }
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
                self.debug_window(ctx, ui);
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.paint {
                self.paint_frame(ctx, ui);
            }
            // self.paint_frame(ctx, ui);
        });
    }
}
