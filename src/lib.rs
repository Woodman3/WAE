#![allow(non_snake_case)]
#![allow(unused)]
#![allow(dead_code)]
use std::{cell::OnceCell, os::raw::c_char, ptr::{null_mut}};
use crate::calculator::Calculator;
use std::ffi::{CString,CStr};
use serde_json::Value;
use timeline::action_to_event;

//mod block;
mod calculator;
mod demo;
mod frame;
mod map;
mod timeline;
mod unit;
mod utils;
mod api;

static mut INSTANCE:OnceCell<Calculator> = OnceCell::new();

#[no_mangle]
pub unsafe extern "C" fn init(path:*const c_char)->u8{
    if path.is_null(){
        println!("pointer can't be null!");
        return 1 
    }
    let cstr = CStr::from_ptr(path);
    if let Ok(path) = cstr.to_str(){
        if let Ok(c) = utils::config::Config::new(path){
            if let Ok(ca) = calculator::Calculator::new(&c){
                if let Ok(_)=INSTANCE.set(ca){
                    return 0
                }
                println!("instance set fail!");
            }else{
                println!("calculator new fail,please check config");
            }
        }else{
            println!("can't load config file");
        }
    }
    println!("can't convert cstring to ruststr");
    1
}

#[no_mangle]
pub unsafe extern "C" fn step()->u8{
    if let Some(c) = INSTANCE.get_mut(){
        if c.step(){
            return 0
        }
        println!("can't step");
    }else{
        println!("can't get instance");
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn get_obs()->*mut c_char{
    if let Some(ca) = INSTANCE.get(){
        let json = ca.get_obs(); 
        if let Ok(r)=CString::new(json.to_string()){
            return r.into_raw()
        }
        println!("can't convert json to CString");
    }else{
        println!("can't get instance");
    }
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn get_acs()->*mut c_char{
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn action(args:*const c_char)->u8{
    if args.is_null(){
        println!("pointer can't be null!");
        return 1 
    }
    let cstr = CStr::from_ptr(args);
    if let Ok(js) = cstr.to_str(){
        if let Ok(json) =serde_json::from_str::<Value>(js){ 
            if let Ok(e) = action_to_event(&json){
                if let Some(Ca) = INSTANCE.get_mut(){
                    Ca.insert_event(e);
                    return 0
                } 
                println!("can't get instance");
            }
            println!("can't convert json to action");
        } 
        println!("can't convert str to json");
    }
    println!("can't convert cstring to ruststr");
    1
}

#[no_mangle]
pub unsafe extern "C" fn free_str(str:*mut c_char)->u8{
    if !str.is_null(){
        drop(CString::from_raw(str));
        return 0
    }
    println!("pointer can't be null!");
    1
}