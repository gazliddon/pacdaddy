use json::{JsonValue};
use v2::V2;

pub struct MyV2(pub V2);

use pickup::{ Pickup, PickupType};

impl<'a> From<&'a PickupType> for JsonValue {
    fn from(v : &'a PickupType) -> JsonValue {
        format!("{:?}", v).into()
    }
}

impl<'a> From<&'a MyV2> for JsonValue {
    fn from(v : &'a MyV2) -> JsonValue {
        object!{
            "x" => v.0.x,
            "y" => v.0.y,
        }
    }
}

impl<'a> From<&'a Pickup> for JsonValue {
    fn from(o : &'a Pickup) -> JsonValue {
        object!{
            "uuid" => o.uuid,
            "pos" => &MyV2(o.pos),
            "time" => o.time,
            "pickupType" => &o.pickup_type,
        }
    }
}

use player::Player;

impl<'a> From<&'a Player> for JsonValue {
    fn from(o : &'a Player) -> JsonValue {
        object!{
            "uuid" => o.uuid,
            "pos" => &MyV2(o.pos),
            "vel" => &MyV2(o.vel),
            "scale" => o.scale,
            "score" => o.score,
            "name" => o.name.clone(),
        }
    }
}


use gamestate::GameState;

impl<'a > From<&'a GameState> for JsonValue {
    fn from(o : &'a GameState) -> JsonValue {

        let players : Vec<&'a Player> = o.players.iter().map(|(_k,v)| v).collect();
        let pickups : Vec<&'a Pickup> = o.pickups.iter().map(|(_k,v)| v).collect();

        let ret = object!{
            "objs" => pickups,
            "time" => 0,
            "players" => players
        };
        ret
    }
}
