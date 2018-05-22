use std::sync::mpsc::{channel, Sender, Receiver };
use messages::Message;

pub struct Coms {
    tx: Sender<Message>,
    rx: Receiver<Message>,
    loopback: Sender<Message>,
}

impl Coms {
    pub fn new(send : Sender<Message>) -> Self {
        let (tx_to_me, rx) = channel();
        Self {
            tx : send,
            rx : rx,
            loopback: tx_to_me,
        }
    }

    pub fn get_sender(&self) -> Sender<Message> {
        self.loopback.clone()
    }

    pub fn send(&self, m : Message) {
        self.tx.send(m).unwrap()
    }
}
