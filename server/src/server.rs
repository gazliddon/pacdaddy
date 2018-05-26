use ws;

use gamestate::{GameState};
use connection::{Connection};
use std::sync::mpsc::{channel, Sender};
use messages::Message;

use network::Connections;

use std::sync::{Arc, Mutex};

pub struct Server {
    tx_to_game_state: Sender<Message>,
    game_state: GameState,
    connections: Arc<Mutex<Connections>>,
}

impl Server {
    pub fn new() -> Server {

        use std;

        let (tx_to_server, rx) = channel();

        let game_state = GameState::new(tx_to_server);
        let tx_to_game_state = game_state.get_sender();


        let connections = Arc::new(Mutex::new(Connections::new()));

        let server = Server { 
            tx_to_game_state, game_state, 
            connections : Arc::clone(&connections),
        };

        let _t1 = std::thread::spawn(move || {
            loop {
                let msg  = rx.recv().unwrap();

                if msg.id == 0 {
                    // broadcast
                } else {

                    let mut unlocked = connections.lock().unwrap();
                    let _res = unlocked.send(msg.id, "jksakjska".to_string());
                    // handle error!
                }
            }
        });

        server
    }
}

impl ws::Factory for Server {
    type Handler = Connection;

    fn client_connected(&mut self, out: ws::Sender) -> Connection {
        let id = {
            let mut unlocked = self.connections.lock().unwrap();
            unlocked.add(out)
        };
        Connection::new(id, self.tx_to_game_state.clone())
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
