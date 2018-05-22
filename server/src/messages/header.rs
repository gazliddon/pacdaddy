use messages::types::*;

pub enum Payload {
    Nothing,
    Unknown(String),
    Hello(String),
    State(GameState),
    Delete(u64),
    Ping,
    Pong(u64),
    PlayerInfo(PlayerInfo),
    PickupInfo(PickupInfo),
    PlayerUpdate(PlayerUpdateInfo),
}

impl Payload {
    pub fn get_name(&self) -> &'static str {
        match *self {
            Payload::Nothing => "nothing",
            Payload::Unknown(_) => "uknown",
            Payload::Hello(_) => "hello",
            Payload::PlayerInfo(_) => "playerInfo",
            Payload::State(_) => "state",
            Payload::Delete(_) => "delete",
            Payload::Ping => "ping",
            Payload::Pong(_) => "pong",
            Payload::PickupInfo(_) => "pickupInfo",
            Payload::PlayerUpdate(_) => "palyerUpdate",
        }
    }
}

pub struct Message {
    pub msg: String,
    pub time: u64,
    pub id: u64,
    pub data: Payload,

}

impl Message {
    pub fn new(data : Payload, id : u64, time: u64 ) -> Self {
        let msg = data.get_name().to_string();
        Self {
            id, time, msg, data
        }
    }

}

