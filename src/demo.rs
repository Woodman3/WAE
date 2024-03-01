// impl DirectSkill {
//     fn schedule(&self,f:Frame){
//
//     }
// }

pub fn fun(){
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{input}");
    match input.trim(){
        "s"=> {
            println!("1");
        }
        "l" =>{
            println!("2");
        }
        &_ => {
            println!("wrong");
        }
    }
}