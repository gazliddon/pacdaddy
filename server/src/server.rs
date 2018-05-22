use gamestate::{GameState};
use std::collections::HashMap;
use connection::{Connection};
use ws;
use std::sync::mpsc::{channel, Sender};
use messages::Message;


struct Connections {
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

    fn add(&mut self, out : ws::Sender) -> u64 {
        let id = self.next_connection_id;
        self.next_connection_id = id + 1;
        self.connections.insert(id,out);
        id
    }

    fn remove(&mut self, id : u64) {
        self.connections.remove(&id);
    }

    fn send(&mut self, id : u64, msg : String) -> ws::Result<()> {
        if let Some(out) = self.connections.get(&id) {
            out.send(msg)?;
        };

        Ok(())
    }
}

pub struct Server {
    tx_to_game_state: Sender<Message>,
    game_state: GameState,
    connections: Connections,
}


impl Server {
    pub fn new() -> Server {

        use std;

        let (tx_to_server, rx) = channel();

        let game_state = GameState::new(tx_to_server);
        let tx_to_game_state = game_state.get_sender();

        let server = Server { 
            tx_to_game_state, game_state,
            connections: Connections::new(),
        };

        let _t1 = std::thread::spawn(move || {
            loop {
                let _msg  = rx.recv().unwrap();
                // dispatch here
            }
        });

        server
    }
}

impl ws::Factory for Server {
    type Handler = Connection;

    fn client_connected(&mut self, out: ws::Sender) -> Connection {
        let id = self.connections.add(out);
        let con = Connection::new(id, self.tx_to_game_state.clone());
        con
    }

    fn connection_made(&mut self, _ws: ws::Sender) -> Connection {
        panic!("no tahanks!")
    }
}

pub fn listen(host : &str, port : u32 ) -> ws::Result<ws::WebSocket<Server>> {
    let server = Server::new();

    info!("Starting a simpleserver on port {}", port);

    let con_str = format!("{}:{}", host, port);
    let ws = ws::WebSocket::new(server)?.bind(con_str)?;

    info!("Bound to {:?}", ws.local_addr());

    Ok(ws)
}
