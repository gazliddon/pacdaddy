use msghdr::MsgHdr;
use ws;
use errors;
use messages::Message;
use std::sync::mpsc::{Sender};

pub struct Connection {
    tx_to_game_state : Sender<Message>,
    id : u64,
}

impl Connection {
    pub fn new(id : u64, tx: Sender<Message> ) -> Self {
        Self { tx_to_game_state: tx, id }
    }

    fn handle_message(&mut self, msg: ws::Message ) -> Result<(), errors::Errors> {
        use jsonparse::{to_v2};

        use messages::{Payload, Message, PlayerUpdateInfo};

        let msg_string = msg.to_string();

        let hdr = MsgHdr::from_str(&msg_string)?;
        let client_id = hdr.get_client_id();
        let client_time = hdr.get_time();

        let payload  = match hdr.get_type() {

            "hello" => {
                let name = hdr.data["name"].to_string();
                Payload::Hello(name)
            }

            "pong" => {
                // TODO
                Payload::Pong(0)
            }

            "playerInfo" => {
                Payload::PlayerUpdate( PlayerUpdateInfo {
                    pos : to_v2(&hdr.data, "pos")?,
                    vel :  to_v2(&hdr.data, "vel")?,
                } )
            }

            _ => {
                Payload::Unknown(hdr.original)
            }
        };

        let message = Message::new(payload, client_id, client_time);
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

