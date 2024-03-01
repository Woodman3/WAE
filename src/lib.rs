use std::cell::OnceCell;
use crate::calculator::Calculator;

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
   0
}

#[no_mangle]
pub unsafe extern "C" fn get_space()->String{
    String::from("0")
}

#[no_mangle]
pub unsafe extern "C" fn direct(_args:String)->u8{
    0
}