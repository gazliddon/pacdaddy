#![allow(dead_code)]

extern crate env_logger;
#[macro_use] extern crate structopt;
#[macro_use] extern crate log;
#[macro_use] extern crate json;

extern crate ws;
extern crate cgmath;
extern crate rand;
extern crate url;

mod pickup;
mod clock;
mod networkobjs;
mod gamestate;
mod server;
mod connection;
mod utils;
mod msgbatch;
mod player;
mod serial;
mod v2;
mod opts;


////////////////////////////////////////////////////////////////////////////////

fn main() {
    use std;
    use std::sync::{ Arc };
    use gamestate::make_gamestate;
    use server::{listen};
    use std::time::Duration;

    let opts = opts::Opts::new();

    let state =  make_gamestate();
    let thread_state = Arc::clone(&state);

    let refresh = Duration::from_millis(opts.get_rate_millis());

    std::thread::spawn(move || {
        loop {
            {
                let mut unlocked_state = thread_state.lock().unwrap();
                unlocked_state.update();
            }
            std::thread::sleep(refresh);
        }
    });

    // A WebSocket echo server
    let host = opts.get_host();
    let port = opts.get_port();

    let ws = listen(&host, port, &state).unwrap();
    ws::WebSocket::run(ws).unwrap();
}
