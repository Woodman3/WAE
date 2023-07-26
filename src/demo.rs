// use crate::timeline::Event;
// trait A{
//     fn f(&self);
//     fn c(&self)->Box<dyn A>;
// }
#[derive(Clone)]
struct Dog{
    v:i32
}
struct Cat{
    v:i32
}

// impl A for Dog {
//     fn f(&self){
//         println!("I'm a Dog ,id {}",self.v);
//     }
//     fn c(&self)->Box<dyn A>{
//         Box::new(self.clone())
//     }
// }
// impl A for Cat {
//     fn f(&self){
//         println!("I'm a Cat ,id {}",self.v);
//     }
//     fn c(&self)->Box<dyn A>{
//         Box::new(self.clone())
//     }
// }
pub fn fun(){
    // let mut v=Vec::<Box<dyn A>>::new();
    // let c=Cat{v:3};
    // v.push(Box::new(d));
    // v.push(Box::new(c));
    // if let Some(e)=v.last(){
    //     let f =e.c();
    //     v.push(f);
    // }
    // for a in &v{
    //     a.f();
    // }
    // for a in v.iter(){
    //     // a.f();
    //     println!("{}",a.v);
    // }

    // let e1=crate::timeline::hostile::EnemyPlaceEvent{
    //     enemy_id:32,
    //     enemy_route:26
    // };
    // let v=Vec::<Box<dyn Event>>::new();
    // v.push(Box::new(e1));
}
