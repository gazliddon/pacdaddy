use ws;

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


        let connections = Arc::new(Mutex::new(Connections::new()));

        let server = Server { 
            tx_to_game_state,
            connections : Arc::clone(&connections),
        };

        let sixty_hertz = time::Duration::from_millis(17);

        let _t0 = thread::spawn(move || {

            loop {
                game_state.update().unwrap();
                
                thread::sleep(sixty_hertz);
            }
        });

        let _t1 = thread::spawn(move || {
            loop {
                // Does the server have anything to say?
                // Should be done in _t0 really?
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


    fn connection_made(&mut self, out: ws::Sender) -> Connection {
        let id = {
            let mut unlocked = self.connections.lock().unwrap();
            unlocked.add(out)
        };

        Connection::new(id, self.tx_to_game_state.clone())
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
