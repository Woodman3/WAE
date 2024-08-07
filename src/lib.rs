#![allow(non_snake_case)]
#![allow(unused)]
#![allow(dead_code)]
use crate::calculator::Calculator;
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::{cell::OnceCell, os::raw::c_char, ptr::null_mut};
use timeline::action_to_event;

//mod block;
pub mod calculator;
mod demo;
pub mod frame;
pub mod map;
pub mod timeline;
pub mod unit;
pub mod utils;

static mut INSTANCE: OnceCell<Calculator> = OnceCell::new();

/// init calculator and make first frame,only shoule be use once
/// # args
/// * `path` - the config directory
#[no_mangle]
pub unsafe extern "C" fn init(path: *const c_char) -> u8 {
    if path.is_null() {
        println!("pointer can't be null!");
        return 1;
    }
    let cstr = CStr::from_ptr(path);
    if let Ok(path) = cstr.to_str() {
        // if let Ok(c) = utils::config::Config::new(path){
        //     if let Ok(ca) = calculator::Calculator::new(&c){
        //         if let Ok(_)=INSTANCE.set(ca){
        //             return 0
        //         }
        //         println!("instance set fail!");
        //     }else{
        //         println!("calculator new fail,please check config");
        //     }
        // }else{
        //     println!("can't load config file");
        // }
        match utils::config::Config::new(path) {
            Ok(c) => {
                if let Ok(ca) = calculator::Calculator::new(&c) {
                    if let Ok(_) = INSTANCE.set(ca) {
                        return 0;
                    }
                    println!("instance set fail!");
                } else {
                    println!("calculator new fail,please check config");
                }
            }
            Err(e) => {
                println!("can't load config file,Error message:{:?}", e);
            }
        }
    }
    println!("can't convert cstring to ruststr");
    1
}

/// step the frame
/// # return value
/// * `0` step gose well
/// * `1` step gose wrong
#[no_mangle]
pub unsafe extern "C" fn step() -> u8 {
    if let Some(c) = INSTANCE.get_mut() {
        if c.step() {
            return 0;
        }
        println!("can't step");
    } else {
        println!("can't get instance");
    }
    1
}

/// get observation space of curent frame
/// return value is a pointer of string,you should use `free_str` to spare space
/// the string is a json
#[no_mangle]
pub unsafe extern "C" fn get_obs() -> *mut c_char {
    if let Some(ca) = INSTANCE.get() {
        if let Some(json) = ca.get_obs() {
            if let Ok(r) = CString::new(json.to_string()) {
                return r.into_raw();
            }
            println!("can't convert json to CString");
        } else {
            println!("can't get json");
        }
    } else {
        println!("can't get instance");
    }
    null_mut()
}

/// get action space of currently frame
/// return value is a pointer of string,you should use `free_str` to spare space
/// the string is a json
#[no_mangle]
pub unsafe extern "C" fn get_acs() -> *mut c_char {
    if let Some(ca) = INSTANCE.get() {
        if let Some(json) = ca.get_acs() {
            if let Ok(r) = CString::new(json.to_string()) {
                return r.into_raw();
            }
            println!("can't convert json to CString");
        } else {
            println!("can't get json");
        }
    } else {
        println!("can't get instance");
    }
    null_mut()
}

/// make action of currently of frame
/// you should alse call `step` after make action
/// # return value
/// * `0` action add well
/// * `1` action add wrong
#[no_mangle]
pub unsafe extern "C" fn action(args: *const c_char) -> u8 {
    if args.is_null() {
        println!("pointer can't be null!");
        return 1;
    }
    let cstr = CStr::from_ptr(args);
    if let Ok(js) = cstr.to_str() {
        if let Ok(json) = serde_json::from_str::<Value>(js) {
            if let Ok(e) = action_to_event(&json) {
                if let Some(Ca) = INSTANCE.get_mut() {
                    Ca.insert_event(e);
                    return 0;
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
pub unsafe extern "C" fn free_str(str: *mut c_char) -> u8 {
    if !str.is_null() {
        drop(CString::from_raw(str));
        return 0;
    }
    println!("pointer can't be null!");
    1
}

#[no_mangle]
pub unsafe extern "C" fn free_instance() {
    INSTANCE.take();
}

#[no_mangle]
pub unsafe extern "C" fn get_frame() -> *mut c_char {
    if let Some(ca) = INSTANCE.get() {
        if let Some(json) = ca.get_frame() {
            if let Ok(r) = CString::new(json.to_string()) {
                return r.into_raw();
            }
            println!("can't convert json to CString");
        } else {
            println!("can't get json");
        }
    } else {
        println!("can't get instance");
    }
    null_mut()
}
