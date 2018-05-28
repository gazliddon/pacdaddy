use v2::{V2};
use messages;

#[derive(Clone)]
pub struct Player {
    pub uuid : u64,
    pub pos : V2,
    pub vel : V2,
    frame: u64,
    last_update: u64,
    pub scale : f64,
    pub score : u64,
    pub name : String,
}

impl Player {
    pub fn new(uuid : u64, time: u64, name : &str, pos: V2) -> Self {
        let name = name.to_string();
        let last_update = time; 

        Self {
            uuid, last_update, name, pos,
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

    pub fn update(&mut self, time : u64, pos: V2, vel : V2) {
        self.pos = pos;
        self.vel = vel;
        self.last_update = time;
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






