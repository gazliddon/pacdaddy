#![allow(dead_code)]

#[macro_use] extern crate structopt;
#[macro_use] extern crate log;
#[macro_use] extern crate json;
extern crate env_logger;
extern crate ws;
extern crate cgmath;
extern crate rand;
extern crate url;

mod pickup;
mod clock;
mod gamestate;
mod server;
mod connection;
mod utils;
mod msgbatch;
mod player;
mod serial;
mod v2;
mod opts;
mod msghdr;
mod jsonparse;
mod errors;
mod rtt;
mod coms;

mod messages;

mod connectable;

////////////////////////////////////////////////////////////////////////////////

fn main() {
    let opts = opts::Opts::new();

    // A WebSocket echo server
    let host = opts.get_host();
    let port = opts.get_port();

    let ws = server::listen(&host, port).unwrap();

    // std::thread::spawn(move || {
    //     loop {
    //         do_to_arcmutex(&thread_state, |mut state| {
    //             state.update();
    //         });

    //         std::thread::sleep(refresh);
    //     }
    // });

    ws::WebSocket::run(ws).unwrap();
}
