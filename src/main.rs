extern crate hyper;
extern crate futures;
extern crate tokio_proto;
#[macro_use]
extern crate serde_json;
extern crate wordcut_engine;
extern crate config;
#[macro_use]
extern crate lazy_static;

mod server;

fn main() {
    server::run_server();
}

    
