use json::JsonValue;
use std::sync::{ Arc, Mutex };
use gamestate::{GameState};
use json;
use ws::{Sender};
use ws;
// use networkobjs::{NetworkObjs};
use utils::{mk_msg};
use obj::{V2, MyV2};

struct RttStats {
    max_samples: usize,
    rtts : Vec<f64>,
    pub average : f64,
    pub max : f64,

}

impl RttStats {
    pub fn new(max_samples: usize) -> Self {
        Self {
            max_samples,
            rtts : vec![],
            average: 0.0,
            max : 0.0,
        }
    }

    pub fn reset(&mut self) {
        panic!("ksjakjskajs")
    }

    pub fn add_rtt(&mut self,  rtt : f64, _time : u64 ) {

        self.rtts.push(rtt);
            
        if self.rtts.len() > self.max_samples {
            self.rtts.resize(self.max_samples, 0.0)
        }

        let sum : f64 = self.rtts.iter().sum();
        let avg = sum / self.rtts.len() as f64;
        self.average = avg;
    }
}



pub struct Connection {
    out : Sender,
    state : Arc<Mutex<GameState>>,
    time : u64,

    rtt: Vec<f64>,
    rtt_avg : f64,
    rtt_peak: f64,
}

impl Connection {
    pub fn new(out : ws::Sender, state: Arc<Mutex<GameState>>) -> Self {
        let time = {
            let mut unlocked = state.lock().unwrap();
            unlocked.clock.now()
        };

        let mut rtt = vec![];
        rtt.reserve(20);

        Self { out, time, state, rtt, rtt_avg: 0.0, rtt_peak: 0.0 }
    }

    pub fn send(&self, msg : &str) -> ws::Result<()> {
        self.out.send(msg)
    }

    pub fn send_msg(&self, msg : &str, data : json::JsonValue) -> ws::Result<()> {
        let msg_string = mk_msg(msg, data, self.time);
        self.send(&msg_string)
    }

    fn add_rtt(&mut self, new_rtt : f64) {
        self.rtt.push(new_rtt);
        if self.rtt.len() > 20 {
            self.rtt.resize(20, 0.0)
        }

        let sum : f64 = self.rtt.iter().sum();
        let avg = sum / self.rtt.len() as f64;
        self.rtt_avg = avg;
    }
}

impl ws::Handler for Connection {

    fn on_open(&mut self, _shake: ws::Handshake) -> ws::Result<()> {
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let mut state = self.state.lock().unwrap();

        self.time = state.clock.now();

        let parsed = json::parse(&msg.to_string()).unwrap();

        let _msg_time = parsed["time"].as_u64().unwrap();

        let msg_str = parsed["msg"].to_string();
        let client_id = parsed["id"].as_u64().unwrap();
        let data = &parsed["data"];

        match msg_str.as_str() {

            "hello" => {
                let pos = V2::new(100.0,100.0);
                let name = data["name"].to_string();

                let id = state.add_player(&name, &pos,  self.time);


                let payload = object!{
                    "id" => id,
                    "pos" => &MyV2(pos),
                    "name" => name,
                };

                let jstate : JsonValue = json::from(&*state);
                self.send_msg("joined", payload)?;
                self.send_msg("state", jstate)?;
            }

            "pong" => {
                // let ping_id = data["id"].as_u64().unwrap();
                let send_time = data["time"].as_u64().unwrap();
                let rtt_ns = state.clock.now() - send_time;
                let rtt = ((rtt_ns / 1000) as f64) / 1000.0;
                // self.add_rtt(rtt);
                info!("{} : rtt millis {}", client_id,  rtt);
            }

            "eatfruit" => {
            }

            "player" => {
                let x = data["pos"]["x"].as_f64().unwrap();
                let y = data["pos"]["y"].as_f64().unwrap();
                state.update_player(client_id, &V2::new(x,y), self.time);
            }

            _ => {
                println!("unhandlded msg {}", msg_str)
            }
        }

        Ok(())
    }
}

