use std::sync::mpsc::{Sender};
use messages::{Message};
use ws;
use errors;

pub struct Connection {
    tx_to_game_state : Sender<Message>,
    connection_id : u64,
}

impl Connection {
    pub fn new(connection_id : u64, tx: Sender<Message> ) -> Self {
        Self { tx_to_game_state: tx, connection_id }
    }

    fn handle_message(&mut self, msg: ws::Message ) -> Result<(), errors::Errors> {
        let msg_string = msg.to_string();
        let message = Message::from_str(&msg_string)?;
        self.tx_to_game_state.send(message).unwrap();
        Ok(())
    }
}

impl ws::Handler for Connection {
    fn on_open(&mut self, _shake: ws::Handshake) -> ws::Result<()> {
        // TODO get a player ID right here!!!
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let res = self.handle_message(msg);
        match res {
            Err(errors::Errors::Sockets(ws)) => Err(ws),
            Ok(()) => Ok(()),
            _ => {panic!("unhandled error TODO")}
        }
    }
}

