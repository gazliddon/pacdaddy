use ws;
use json;

use gamestate::{GameState};
use network::{Connection, Connections};
use std::sync::mpsc::{channel, Sender};
use messages::Message;

use std::sync::{Arc, Mutex};

pub struct Server {
    tx_to_game_state: Sender<Message>,
    connections: Arc<Mutex<Connections>>,
}

impl Server {
    pub fn new() -> Server {

        use std::{thread, time};

        let (tx_to_server, rx) = channel();

        let mut game_state = GameState::new(tx_to_server);
        let tx_to_game_state = game_state.get_sender();


        let sixty_hertz = time::Duration::from_millis(17);


        thread::Builder::new().name("update".to_string()).spawn(move || {
            loop {
                game_state.update().unwrap();
                thread::sleep(sixty_hertz);
            }
        }).unwrap();

        let connections = Arc::new(Mutex::new(Connections::new()));
        let thread_cons = Arc::clone(&connections);

        thread::Builder::new().name("connection sink".to_string()).spawn(move || {
            loop {
                // Does the server have anything to say?
                // Should be done in _t0 really?
                let msg  = rx.recv().unwrap();
                info!("Sendning! {:?}", msg);

                let mut unlocked = thread_cons.lock().unwrap();

                let msg_str = json::from(&msg).to_string();

                if msg.id == 0 {
                    // TODO handle error!
                    let _res = unlocked.broadcast(msg_str);
                } else {
                    // TODO handle error!
                    let _res = unlocked.send(msg.id, msg_str);
                }
            }
        }).unwrap();

        Server { 
            tx_to_game_state,connections,
        }
    }
}

impl ws::Factory for Server {
    type Handler = Connection;

    fn connection_made(&mut self, out: ws::Sender) -> Connection {

        use messages::{Payload, Message};

        let id = {
            let mut unlocked = self.connections.lock().unwrap();
            unlocked.add(out.clone())
        };

        let msg = Message::new(Payload::MadeConnection, id, 0);
        let jmsg : json::JsonValue = json::from(&msg);
        out.send(jmsg.to_string()).unwrap();

        Connection::new(id, self.tx_to_game_state.clone())
    }
}

pub fn listen(host : &str, port : u32 ) -> ws::Result<ws::WebSocket<Server>> {
    let server = Server::new();
    let con_str = format!("{}:{}", host, port);
    let ws = ws::WebSocket::new(server)?.bind(con_str)?;

    info!("Bound to {:?}", ws.local_addr());

    Ok(ws)
}
