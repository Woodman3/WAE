use super::*;
use serde_json::to_string_pretty;

#[test]
fn test_serialize(){
    let mut m=Map::default();
    m.width=2;
    m.height=3;
    m.enemy = vec![vec![Vec::<Weak<RefCell<Enemy>>>::new();m.width as usize];m.height as usize];
    m.operator =vec![vec![None;m.width as usize];m.height as usize];
    m.operator[1][1]=Some("skadi".into());
    let json = to_string_pretty(&m).unwrap();
    println!("{json}");
}