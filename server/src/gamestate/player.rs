use v2::{V2};
use messages;

#[derive(Clone)]
pub struct Player {
    pub uuid : u64,
    pub pos : V2,
    pub vel : V2,
    frame: u64,
    pub last_update: u64,
    pub scale : f64,
    pub score : u64,
    pub name : String,
    pub time : u64, 
}

impl Player {
    pub fn new(uuid : u64, server_time : u64, client_time: u64, name : &str, pos: V2) -> Self {
        let name = name.to_string();

        Self {
            uuid, 
            last_update : server_time, 
            time : client_time,
            name, pos,
            scale : 1.0,
            vel: V2::new(0.0, 0.0),
            score: 0,
            frame: 0,
        }
    }

    pub fn since_last_update(&self, now : u64) -> f64 {
        (now - self.last_update) as f64 / 1_000_000_000.0
    }

    // TODO TRAIT
    // TODO Make radius count
    pub fn did_collide(&self, pos : &V2, _radius : f64) -> bool {
        use cgmath::prelude::*;
        self.pos.distance(*pos) < (10.0 * self.scale)
    }

    pub fn increase_scale(&mut self) {
        self.scale = self.scale + 0.05;
    }

    pub fn add_points(&mut self, points : u64) {
        self.score = self.score + points
    }

    pub fn update(&mut self, server_time : u64, _client_time : u64, pos: V2, vel : V2) {
        self.pos = pos;
        self.vel = vel;
        self.last_update = server_time;
    }
}

impl<'a> From<&'a Player> for messages::PlayerInfo {
    fn from(p : &'a Player) -> messages::PlayerInfo {
        messages::PlayerInfo {
            uuid : p.uuid,
            pos: p.pos.clone(),
            vel: p.vel.clone(),
            frame: 0,
            score: p.score,
            scale: p.scale,
            name: p.name.clone(),
        }
    }
}






