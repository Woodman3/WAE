use nannou::prelude::*;

struct Model{}
pub fn fun(){
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
fn model(app:&App)->Model{
    Model{}
}
fn update(_app:&App,_model:&mut Model,_update:Update){

}
fn view(app: &App,_model:& Model, frame: Frame) {
    let  draw=app.draw();
    let sin=app.time.sin();
    let points=(0..=300).map(|i|{
        let x=i as f32 - 150.0;
        let point = pt2(x,x.sin())*30.0;
        (point,STEELBLUE)
    });
    draw.polyline()
        .weight(3.0)
        .points_colored(points);
    draw.to_frame(app,&frame).unwrap();
}

