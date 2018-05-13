#![allow(dead_code)]

extern crate env_logger;
#[macro_use] extern crate structopt;

#[macro_use] extern crate log;
#[macro_use] extern crate json;

extern crate ws;
extern crate cgmath;
extern crate rand;
extern crate url;

mod obj;
mod clock;
mod networkobjs;
mod gamestate;
mod server;
mod connection;
mod utils;

use json::JsonValue;

use std::sync::{ Arc, Mutex };
use gamestate::{GameState};
use std::thread;
use connection::{Connection};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opts {
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8, 

    /// admin_level to consider
    #[structopt(short = "h", long = "host")]
    host: Option<String>,

     /// Number of cars
    #[structopt(short = "p", long = "port")]
    port: Option<u32>,
}

pub struct Server {
    state: Arc<Mutex<GameState>>,
    broadcaster : ws::Sender,
    ping_id : u64,
}

impl Server {
    pub fn new(con_str : &str) -> Server {

        let url = url::Url::parse(con_str).unwrap();

        let raw_state = GameState::new();

        let state =  Arc::new(Mutex::new(raw_state));

        let ws_state = Arc::clone(&state);

        let ws = ws::WebSocket::new( move |out | {
            Connection::new(out, Arc::clone(&ws_state))
        }).unwrap();

        let broadcaster = ws.broadcaster();

        let _th = thread::spawn(move || {
            ws.listen(url).unwrap();
        });

        Server {
            state: Arc::clone(&state),
            broadcaster, 
            ping_id: 0,
        }
    }

    pub fn update(&mut self) {
        {
            let mut state = self.state.lock().unwrap();
            let _time = state.update();
        }

        let jstate : JsonValue = {
            let state = self.state.lock().unwrap();
            json::from(&*state)
        };

        self.broadcast("state", jstate).unwrap();
    }

    pub fn get_time(&self) -> u64 {
        let time = {
            let state = self.state.lock().unwrap();
            state.clock.now()
        };
        time
    }

    pub fn broadcast(&mut self, msg : &str, j : JsonValue ) -> ws::Result<()> {
        let time = self.get_time();
        let msg_string = utils::mk_msg(msg, j, time);
        self.broadcaster.send(msg_string)
    }

    pub fn ping(&mut self) -> ws::Result<()> {
        let pmsg = object!{
            "id" => self.ping_id,
        };

        self.ping_id += 1;
        self.broadcast("ping",pmsg)
    }
}

////////////////////////////////////////////////////////////////////////////////

fn main() {
    use std::env;
    use std::time;

    let opts = Opts::from_args();

    if opts.verbose > 0 {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    // A WebSocket echo server
    let port = opts.port.unwrap_or(6502);
    let host = opts.host.unwrap_or("localhost".to_string());

    let con_str = format!("ws:/{}:{}", host, port);

    info!("Starting a simpleserver on port {}", port);

    let mut server = Server::new(&con_str);

    let  pause_time = time::Duration::from_millis(300);

    loop {
        server.update();
        thread::sleep(pause_time);
    }
}
