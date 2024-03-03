use std::{cell::OnceCell, os::raw::c_char};
use crate::calculator::Calculator;
use std::ffi::{CString,CStr};

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
pub unsafe extern "C" fn init(s:String)->u8{
    if let Ok(c) = utils::config::Config::new(s){
        if let Ok(ca) = calculator::Calculator::new(&c){
            if let Ok(_)=INSTANCE.set(ca){
                0
            }else {
                3
            }
        }else{
            2
        }
    }else{
        1
    }
}

#[no_mangle]
pub unsafe extern "C" fn step()->u8{
    if let Some(c) = INSTANCE.get_mut(){
        if c.step(){
            0
        }else{
            1
        }
    }else{
        2
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_obs()->*mut c_char{
    let r=CString::new("test").unwrap();
    r.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn action(args:CStr)->u8{
    0
}

#[no_mangle]
pub unsafe extern "C" fn free_str(str:*mut c_char){
    if !str.is_null(){
        // CString::from_raw(str);
        drop(CString::from_raw(str));
    }
}