use json::JsonValue;


////////////////////////////////////////////////////////////////////////////////

struct MsgBatch {
    dest: Destination,
    messages : Vec<JsonValue>,
}

impl MsgBatch {
    pub fn new( dest: Destination) -> Self {
        Self {
            dest, messages : vec![]
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, PartialEq)]
pub enum Destination {
    Broadcast, 
    Connection(u64)
}

pub struct MsgBatcher {
    batches : Vec<MsgBatch>,
}

impl MsgBatcher {
    pub fn new() -> Self {
        Self {
            batches : vec![]
        }
    }

    // pub fn flush(&mut self, out : &mut Transport)  {
    //     use self::Destination::*;

    //     for b in &self.batches {
    //         let msg : String = object!{
    //             "msg" => "batch",
    //             "data" => b.messages.clone()
    //         }.dump();

    //         match b.dest {
    //             Broadcast => out.broadcast(msg),
    //             Connection(n) => out.send(n, msg),
    //         };

    //     };
    //     self.batches = vec![];
    // }

    fn add_new_batch(&mut self, dest : Destination, data : JsonValue) {
        let mut batch = MsgBatch::new(dest);
        batch.messages.push(data);
        self.batches.push(batch);
    }

    pub fn send_dest(&mut self, dest : Destination, msg_type : &str, data : JsonValue) {
        let data = object!{
            "msg" => msg_type,
            "data" => data
        };

        if self.batches.len() == 0  {
            self.add_new_batch(dest, data);
        } else {
            if self.batches[0].dest != dest {
                self.add_new_batch(dest, data);
            } else {
                self.batches[0].messages.push(data)
            }
        }
    }

    pub fn send(&mut self,  id : u64, msg_type : &str, data : JsonValue) {
        self.send_dest(Destination::Connection(id), msg_type, data)
    }

    pub fn broadcast(&mut self,  msg_type : &str, data : JsonValue) {
        self.send_dest(Destination::Broadcast, msg_type, data)
    }
}


