use v2::V2;

pub struct HelloInfo {
    pub name : String, 
}

pub struct PlayerInfo {
    pub uuid : u64,
    pub pos: V2,
    pub frame: u32,
    pub score: u64,
    pub scale: f64,
    pub name: String,
}

pub struct PickupInfo {
    pub uuid: u64,
    pub kind: String,
    pub pos: V2,
}

pub struct GameStateInfo {
    pub players: Vec<PlayerInfo>,
    pub pickups: Vec<PickupInfo>,
}

pub struct PlayerUpdateInfo {
    pub pos : V2,
    pub vel : V2,
}


