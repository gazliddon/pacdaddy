use json::JsonValue;
use json;
use serial::MyV2;

use messages::{
    Message, 
    Payload,
    HelloInfo,
    PlayerInfo,
    PickupInfo,
    DeleteInfo,
    PongInfo,
    GameStateInfo,
    PlayerUpdateInfo,
    PlayerJoinedInfo,
};

impl<'a> From<&'a Message> for JsonValue {
    fn from(o : &'a Message) -> JsonValue {
        object!{
            "msg" => o.msg.clone(),
            "id" => o.id,
            "time" => o.time,
            "data" => &o.data,
        }
    }
}

impl <'a> From<&'a Payload> for JsonValue {
    fn from(o : &'a Payload) -> JsonValue {

        match o {
            &Payload::Nothing | &Payload::Ping | &Payload::MadeConnection => object!{},

            &Payload::Unknown(ref text) => object!{ "text" => text.clone()},

            &Payload::Hello(ref hello_info) => json::from(hello_info),
            &Payload::PlayerInfo(ref player_info) => json::from(player_info),
            &Payload::State(ref state) => json::from(state),
            &Payload::Delete(ref delete) => json::from(delete),
            &Payload::PlayerDelete(ref delete) => json::from(delete),
            &Payload::Pong(ref pong) => json::from(pong),
            &Payload::PickupInfo(ref pickup_info) => json::from(pickup_info),
            &Payload::PlayerUpdate(ref player_update) => json::from(player_update),
            &Payload::PlayerJoined(ref player_joined) => json::from(player_joined),
        }
    }
}

impl <'a> From<&'a HelloInfo> for JsonValue {
    fn from(_o : &'a HelloInfo) -> JsonValue {
        panic!()
    }
}

impl <'a> From<&'a PlayerInfo> for JsonValue {

    fn from(player : &'a PlayerInfo) -> JsonValue {
        object!{
            "uuid" => player.uuid,
            "pos"  => &MyV2(player.pos.clone()),
            "vel"  => &MyV2(player.vel.clone()),
            "frame"  => player.frame,
            "score"  => player.score,
            "scale"  => player.scale,
            "name"  => player.name.clone(),
        }
    }
}

impl <'a> From<&'a PickupInfo> for JsonValue {
    fn from(pickup_info : &'a PickupInfo) -> JsonValue {
        object!{
            "uuid" => pickup_info.uuid,
            "pos"  => &MyV2(pickup_info.pos.clone()),
            "kind" => pickup_info.kind.clone(),
        }
    }
}

impl <'a> From<&'a GameStateInfo> for JsonValue {
    fn from(_state : &'a GameStateInfo) -> JsonValue {
        // object!{
        //     "players" => state.players,
        //     "pickups" => state.pickups,
        // }
        panic!("sakjajsa")
    }
}

impl <'a> From<&'a PlayerUpdateInfo> for JsonValue {
    fn from(_player_update : &'a PlayerUpdateInfo) -> JsonValue {
        panic!()
    }
}

impl <'a> From<&'a DeleteInfo> for JsonValue {
    fn from(delete_info : &'a DeleteInfo) -> JsonValue {
        object!{
            "toDelete" => delete_info.to_delete,
        }
    }
}

impl <'a> From<&'a PongInfo> for JsonValue {
    fn from(_o : &'a PongInfo) -> JsonValue {
        panic!()
    }
}

impl <'a> From<&'a PlayerJoinedInfo> for JsonValue {
    fn from(_o : &'a PlayerJoinedInfo) -> JsonValue {
        object!{
            "uuid" => _o.uuid,
            "pos" => &MyV2(_o.pos.clone()),
        }
    }
}


