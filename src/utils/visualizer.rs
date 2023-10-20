use std::fmt::format;
use eframe::egui;
use crate::frame::Frame;
use eframe::egui::{Context, Painter, pos2, Pos2, Rangef, Rect, Sense, Stroke, TextFormat, Ui, vec2, WidgetText};
use eframe::egui::ImageData::Color;
use eframe::egui::MouseWheelUnit::Point;
use eframe::egui::Shape::Circle;
use eframe::egui::WidgetText::{LayoutJob, RichText};
use eframe::epaint::{CircleShape, Color32};
use crate::calculator::Calculator;
use super::visualizer_config::*;
use super::math;

pub struct Visualizer{
    pub c:Calculator,
    run:bool,
}

fn paint_frame(f:&Frame,painter:Painter){
    let width=f.map.width ;
    let height = f.map.height;
    let map_stroke:Stroke=(3.0,Color32::BLACK).into();
    for i in 0..=height {
        painter.hline(Rangef{min:0.0,max:width as f32 *BLOCK_SIZE},i as f32 *BLOCK_SIZE,map_stroke);
    }
    for i in 0..=width{
        painter.vline(i as f32*BLOCK_SIZE,Rangef{min:0.0,max:height as f32 *BLOCK_SIZE},map_stroke);
    }
    let enemy_stroke:Stroke=(4.0,ENEMY_COLOR).into();
    for e in f.enemy_set.iter(){
        let e=e.borrow();
        let (mut x,mut y):(f32,f32)=e.location.into();
        x*=BLOCK_SIZE;
        y*=BLOCK_SIZE;
        painter.circle_stroke(pos2(x,y), ENEMY_RADIUS, enemy_stroke);
    }
    for (key,o) in f.operator_deploy.iter(){
        let o =o.borrow();
        let p=super::math::Point::from(o.location);
        let (mut x,mut y):(f32,f32)=p.into();
        x*=BLOCK_SIZE;
        y*=BLOCK_SIZE;
        painter.circle_filled(pos2(x,y),OPERATOR_RADIUS,OPERATOR_COLOR);
    }
    for b in f.bullet_set.iter(){
        let (mut x,mut y):(f32,f32)=b.location.into();
        x*=BLOCK_SIZE;
        y*=BLOCK_SIZE;
        painter.circle_filled(pos2(x,y),BULLET_RADIUS,BULLET_COLOR);
    }
    let me = &f.map.enemy;
    for i in 0..width{
        for j in 0..height{
            let l=me[j as usize][i as usize].len();
            if l!=0{
                let pos=Pos2::from([(i as f32)*BLOCK_SIZE+5.0,(j as f32)*BLOCK_SIZE+5.0]);
                painter.circle_filled(pos,3.0,Color32::BROWN);
            }
        }
    }

}
fn paint_info(f:&Frame,ui:&mut Ui){
    // let text=RichText("a text".into());
    let mut info =egui::text::LayoutJob::default() ;
    let text_format=TextFormat::default();
    let text = format!("time_stamp:{}\n",f.timestamp);
    info.append(text.as_str(),0.0,text_format.clone());
    for e in f.enemy_set.iter(){
        let e=e.borrow();
        let text = format!("{e}\n");
        info.append(text.as_str(),0.0,text_format.clone());
    };
    for (name,o) in f.operator_deploy.iter(){
        let o =o.borrow();
        let text=format!("{name},{o}\n");
        info.append(text.as_str(),0.0,text_format.clone());
    }
    for b in f.bullet_set.iter(){
        let text = format!("{b}");
        info.append(text.as_str(),0.0,text_format.clone());
    }
    ui.label(info);
}
impl Visualizer{
    pub fn new(cc: &eframe::CreationContext<'_>,c:Calculator)->Self{
        Self{
            c,
            run:false,
        }
    }

}
impl eframe::App for Visualizer{
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        egui::Context::request_repaint(ctx);
        egui::CentralPanel::default().show(ctx,|ui|{
            let r:Rect=[(100.0,100.0).into(),(2000.0,2000.0).into()].into();
            // ui.allocate_rect(r,Sense::hover());
            // let painter=Painter::new(
            //     ctx.clone(),
            //     ui.layer_id(),
            //     // ui.available_rect_before_wrap(),
            //     r,
            // );
            let (_r,painter)=ui.allocate_painter(vec2(1000.0,2000.0),Sense::hover());
            ui.label(format!("rect min:{:?},max{:?}",r.min,r.max));
            paint_frame(&self.c.frame_vec[0],painter);
        });
        egui::SidePanel::right("debug")
            .min_width(300.0)
            .show(ctx,|ui|{
                ui.checkbox(&mut self.run,"run");
                if ui.button("next").clicked() || self.run{
                    self.c.next();
                };
                paint_info(&self.c.frame_vec[0],ui);
        });
    }
}