use ws;

use std::collections::HashMap;

pub struct Connections {
    connections: HashMap<u64, ws::Sender>,
    next_connection_id : u64,
}

impl Connections {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            next_connection_id: 1
        }
    }

    pub fn add(&mut self, out : ws::Sender) -> u64 {
        let id = self.next_connection_id;
        self.next_connection_id = id + 1;
        self.connections.insert(id,out);
        id
    }

    pub fn remove(&mut self, id : u64) {
        self.connections.remove(&id);
    }

    // TODO made this my error

    pub fn send(&mut self, id : u64, msg : String) -> ws::Result<()> {
        if let Some(out) = self.connections.get(&id) {
            out.send(msg)?;
        };

        Ok(())
    }

    pub fn get<'a>(&'a self, _id: u64) -> Option<&'a ws::Sender> {
        panic!("sklajsa")
    }
}

