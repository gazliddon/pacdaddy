use json::JsonValue;
use ws::{Sender};
use ws;

pub struct MsgBatch {
    batch : Vec<(Destination, JsonValue)>,
}

pub enum Destination {
    Broadcast, 
    Connection(u64)
}


impl MsgBatch {
    pub fn new() -> Self {
        Self {
            batch : vec![],
        }
    }

    pub fn flush(&mut self, _out : &Sender, _time : u64) -> ws::Result<()> {
        // TODO write this shit
        panic!("");
        // let msg = object!{
        //     "msg" => "batch",
        //     "time" => time,
        //     "data" => self.batch.clone(),
        // };

        // self.batch = vec![];
        // out.send(msg.to_string())
    }

    pub fn send(&mut self, dest : Destination, msg_type : &str, data : JsonValue) {
        let msg = object!{
            "msg" => msg_type.clone(),
            "data" => data
        };
        self.batch.push((dest, msg))
    }

    pub fn broadcast(&mut self,  msg_type : &str, data : JsonValue) {
        self.send(Destination::Broadcast, msg_type, data)
    }
 
}

