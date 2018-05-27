use gamestate::state::GameState;
use messages::{Message, Payload};
use std::sync::mpsc::{ Sender};
use errors::Errors;

// Message sending / receiving
impl GameState {

    fn get_next_msg(&self) -> Result<Message, Errors> {
        // use std::sync::mpsc::TryRecvError;
        let msg = self.rx_from_server.try_recv()?;
        Ok(msg)
    }

    fn dispatch_message(&mut self, msg : Message) -> Result<(), Errors> {
        use messages::Payload::*;

        match msg.data {
            Hello(_hello_payload) => {
                info!("ignoring hello");
                Ok(())
            }

            PlayerInfo(_player_info) => {
                info!("ignoring player info");
                Ok(())

            },

            Pong(_pongfo) => {
                info!("ignoring pong info");
                Ok(())
            },

            _ => {
                Err(Errors::UnhandledMessage(format!("{:?}", msg.data)))
            }
        }
    }

    pub fn process_messages(&mut self) -> Result<usize, Errors> {
        let mut msgs_handled = 0;

        loop {
            let msg = self.get_next_msg();

            match msg {
                Err(Errors::ChannelEmpty) => break,

                Ok(m) => {
                    self.dispatch_message(m)?;
                    msgs_handled = msgs_handled + 1;
                }

                Err(e) => {
                    return Err(e);
                }

            }
        }

        Ok(msgs_handled)
    }

    pub fn get_sender(&self) -> Sender<Message> {
        self.tx_to_me.clone()
    }

    pub fn broadcast(&self, data : Payload) {
        self.send(0, data)
    }

    pub fn send(&self, id : u64, data : Payload ) {
        let _message = Message::new(data, id, 0);
    }
}
