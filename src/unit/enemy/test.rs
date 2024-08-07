use super::*;
use serde_json::to_string_pretty;

#[test]
fn test_serialize() {
    let e = Enemy::default();
    let json = to_string_pretty(&e).unwrap();
    println!("{}", json);
}
