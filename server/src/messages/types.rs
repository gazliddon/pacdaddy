use v2::V2;

pub struct PlayerInfo {
    pub id : u64,
    pub pos: V2,
    pub frame: u32,
    pub score: u64,
    pub scale: f64,
    pub name: String,
}

pub struct PickupInfo {
    pub id: u64,
    pub kind: String,
    pub pos: V2,
}

pub struct GameState {
    pub players: Vec<PlayerInfo>,
    pub pickups: Vec<PickupInfo>,
}

pub struct PlayerUpdateInfo {
    pub pos : V2,
    pub vel : V2,
}


