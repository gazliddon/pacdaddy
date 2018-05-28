use v2::V2;

#[derive(Debug)]
pub struct HelloInfo {
    pub name : String, 
}

#[derive(Debug)]
pub struct PlayerInfo {
    pub uuid : u64,
    pub pos: V2,
    pub vel: V2,
    pub frame: u32,
    pub score: u64,
    pub scale: f64,
    pub name: String,
}

#[derive(Debug)]
pub struct PickupInfo {
    pub uuid: u64,
    pub kind: String,
    pub pos: V2,
}

#[derive(Debug)]
pub struct GameStateInfo {
    pub players: Vec<PlayerInfo>,
    pub pickups: Vec<PickupInfo>,
}

#[derive(Debug)]
pub struct PlayerUpdateInfo {
    pub pos : V2,
    pub vel : V2,
}

#[derive(Debug)]
pub struct PongInfo {
    pub send_time : u64,
}

#[derive(Debug)]
pub struct DeleteInfo {
    pub to_delete : u64,
}


