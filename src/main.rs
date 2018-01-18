#![no_main]
#![feature(link_args)]
#![link_args = "-Wl,--subsystem,windows"]

extern crate hyper;
extern crate futures;
extern crate tokio_proto;
#[macro_use]
extern crate serde_json;
extern crate wordcut_engine;
extern crate config;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate winservice;

mod server;

use std::os::raw::{c_char, c_int, c_void};
use std::sync::mpsc::Receiver;

use std::ffi::CStr;
use std::sync::Mutex;
use std::cell::RefCell;

lazy_static! {
    static ref CONF_PATH: Mutex<RefCell<String>> = {
        Mutex::new(RefCell::new(String::from(".")))
    };
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn  WinMain(hInstance : *const c_void, hPrevInstance : *const c_void,
    lpCmdLine : *const c_char, nCmdShow : c_int) -> c_int
{
    
    let arg = unsafe { CStr::from_ptr(lpCmdLine).to_str().unwrap() };

    {
        let tmp_conf_path = CONF_PATH.lock().unwrap();
        let mut mut_conf_path = tmp_conf_path.borrow_mut();
        mut_conf_path.clear();
        mut_conf_path.push_str(arg);
    }

    Service!("myService", service_main)
}

fn service_main(args : Vec<String>, end : Receiver<()>) -> u32 {
    //if let Ok(_) = end.try_recv() { break; }
    let tmp_conf_path = CONF_PATH.lock().unwrap();
    server::run_server(&tmp_conf_path.borrow()[..]);
    0
}
