use messages::payloads::*;

#[derive(Debug, Clone)]
pub enum Payload {
    Nothing,
    Unknown(String),
    Hello(HelloInfo),
    State(GameStateInfo),
    Delete(DeleteInfo),
    PlayerDelete(DeleteInfo),
    Ping,
    Pong(PongInfo),
    PlayerInfo(PlayerInfo),
    PickupInfo(PickupInfo),
    PlayerUpdate(PlayerUpdateInfo),
    PlayerJoined(PlayerJoinedInfo),
    MadeConnection,
}

#[derive(Debug, Clone)]
pub enum AllowedUsage {
    FromServerOnly,
    FromClientOnly,
}

#[derive(Debug, Clone)]
pub struct MetaData {
    id : &'static str,
    allowed_usage: AllowedUsage,
}

#[derive(Debug, Clone)]
pub struct PayloadStruct {
    data: Payload,
    meta_data: &'static MetaData,
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
            Payload::PlayerUpdate(_) => "playerUpdate",
            Payload::MadeConnection => "madeConnection",
            Payload::PlayerDelete(_) => "playerDelete",
            Payload::PlayerJoined(_) => "playerJoined",
        }
    }
}

#[derive(Debug, Clone)]
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
