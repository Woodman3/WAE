use std::{any::Any, collections::HashMap};
use regex::Regex;
use serde::de;

#[derive(Debug)]
struct Frame{
    a:Vec<A>, 
    b:HashMap<String,B>
}

#[derive(Debug)]
struct A{
    val:i32
}

#[derive(Debug)]
struct B{
    val:i32
}

#[derive(Debug)]
enum Pointer<'a>{
    Frame(&'a Frame),
    A(&'a A),
    B(&'a B),
    Val(&'a i32)
}

#[test] 
fn fun(){
    let mut f = Frame{
        a:vec![A{val:1},A{val:2}],
        b:HashMap::new()
    };    
    f.b.insert("0".to_string(),B{val:3});
    let input = "p a[0].val";
    let re = Regex::new(r"^\s*(\w+)\s*(.*)").unwrap();
    if let Some(caps) = re.captures(input){
        let command = caps.get(1).unwrap().as_str();
        let object = caps.get(2).unwrap().as_str();
        match command{
            "p" => {
                let mut obj = Pointer::Frame(&f);
                for field in object.split('.'){
                    if field.ends_with("]") {
                        let re = Regex::new(r"(\w+)\[(\d+)\]").unwrap();
                        if let Some(caps) = re.captures(field){
                            let field = caps.get(1).unwrap().as_str();
                            let mut index = caps.get(2).unwrap().as_str();
                            match field{
                                "a" => {
                                    let index: usize = index.parse().unwrap();
                                    obj = match obj{
                                        Pointer::Frame(obj) => Pointer::A(&obj.a[index]),
                                        _ => {panic!("Invalid field: {}",field);}
                                    };
                                },
                                "b" => {
                                    obj = match obj{
                                        Pointer::Frame(obj) => Pointer::B(&obj.b[index]),
                                        _ => {panic!("Invalid field: {}",field);}
                                    };
                                },
                                _ => {
                                    panic!("Invalid field: {}",field);
                                }
                            }
                        }
                    }else{
                        match field{
                            "val" =>{
                                obj = match obj{
                                    Pointer::A(obj) => Pointer::Val(&obj.val),
                                    Pointer::B(obj) => Pointer::Val(&obj.val),
                                    _ => {panic!("Invalid field: {}",field);}
                                }
                            }
                            _ => {
                                panic!("Invalid field: {}",field);
                            }
                        }
                    }
                }
                println!("{:?}",obj);
            },
            _ => {
                panic!("Invalid command: {}",command);
            }
        }
    }
}
