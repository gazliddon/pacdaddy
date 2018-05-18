use std::collections::HashMap;
use json::JsonValue;
use rand;
use pickup::{Pickup, PickupType};
use clock;
use v2::{V2};

use msgbatch::{MsgBatch};
use player::{Player};
use json;
use std::sync::{ Arc, Mutex };

fn mk_random_vec() -> V2 {
    use rand::distributions::{IndependentSample, Range};
    let between = Range::new(0.0f64, 1000.0);
    let mut rng = rand::thread_rng();
    let x = between.ind_sample(&mut rng);
    let y = between.ind_sample(&mut rng);
    V2::new(x,y)
}

fn mk_random_pickup(time : u64) -> Pickup {
    use rand::Rng;

    let names = vec![
        PickupType::Coke,
        PickupType::Pizza,
        PickupType::Burger,
    ];

    let obj_type = rand::thread_rng().choose(&names).unwrap();

    Pickup::new(obj_type.clone(), 0, mk_random_vec(), time)
}


////////////////////////////////////////////////////////////////////////////////

pub struct GameState {
    pub players: HashMap<u64, Player>,
    new_next_id : u64,
    pub clock: clock::Clock,
    pub pickups : HashMap<u64, Pickup>,
    msg_batch : MsgBatch,
}

impl GameState {

    pub fn broadcast(&mut self, msg : &str, jdata :  JsonValue) {
        self.msg_batch.broadcast(msg, jdata)
    }

    pub fn send(&mut self, player_id : u64, msg : &str, jdata :  JsonValue) {
        use msgbatch::Destination::*;
        self.msg_batch.send(Connection(player_id), msg, jdata)
    }

    pub fn get_nw_id(&mut self) -> u64 {
        let ret = self.new_next_id;
        self.new_next_id = self.new_next_id + 1;
        ret
    }

    pub fn add_random_pickup(&mut self, time : u64) -> u64 {
        let p = mk_random_pickup(time);
        self.add_pickup(p)
    }

    pub fn add_pickup(&mut self, pickup : Pickup) -> u64 {
        let mut pickup = pickup.clone();
        let id = self.get_nw_id();
        pickup.id = id;
        let json = json::from(&pickup);
        self.pickups.insert(id, pickup);
        self.broadcast("newPickup",json);
        id
    }

    pub fn remove_pickup(&mut self, id : u64) {
        if let Some(pickup) = self.pickups.get(&id) {
            info!("removing pickup: {}, type: {:?}", id, pickup.pickup_type );
        } else {
            warn!("failure to remove pickup {}", id );
        }

        let _ = self.pickups.remove(&id);
        self.broadcast("removePickup", object!{ "id" => id });
    }

    pub fn add_player(&mut self, name: &str, pos : &V2, time : u64) -> u64 {
        let id = self.get_nw_id();
        let player = Player::new(id, time, name, *pos);
        let pjson : JsonValue = json::from(&player);
        self.players.insert(id, player);

        self.send(id, "hello", pjson);
        let jstate : JsonValue = json::from(&*self);
        self.broadcast("state", jstate);
        id
    }

    pub fn remove_player(&mut self, id : u64) {
        info!("deleting player {}", id);
        self.players.remove(&id).unwrap();
        self.broadcast("removePlayer", object!{ "id" => id });
    }

    pub fn get_player_json(&self, player_id : u64) -> Option<JsonValue> {
        let player = self.players.get(&player_id)?;
        Some(json::from(player))
    }

    pub fn change_player(&mut self, id : u64, func : &Fn(&mut Player) ) {
        if let Some(p) = self.players.get_mut(&id) { 
            func(p);
        }

        if let Some(pjson) = self.get_player_json(id) {
            self.broadcast("changePlayer", pjson);
        }
    }
}

impl GameState {

    pub fn new() -> Self {
        let mut ret = Self {
            players: HashMap::new(),
            new_next_id : 0,
            clock: clock::Clock::new(),
            pickups: HashMap::new(),
            msg_batch : MsgBatch::new(),
        };

        let time = ret.clock.now();

        for _ in 0..100 {
            ret.add_random_pickup(time);
        }

        ret
    }

    fn prune_inactive_players(&mut self, time : u64) {
        let to_kill : Vec<u64> = self.players.iter().filter(|&(_,p)| {
            let since_last = p.since_last_update(time);
            since_last > 3.0
        }).map(|(k,_p)| *k).collect();

        for id in to_kill {
            self.remove_player(id);
        }
    }

    fn collide_pickups(&mut self, _time : u64) {

        // TODO REVIEW this is very suspect
        let mut pickup_hit : Vec<(u64, u64)> = vec![];

        for (object_id, obj) in self.pickups.iter() {
            for (player_id, player) in self.players.iter() {
                if player.did_collide(&obj.pos, 0.0) {
                    pickup_hit.push((*player_id, *object_id));
                    break;
                }
            }
        }

        for &(player_id, pickup_id) in pickup_hit.iter() {
            self.remove_pickup(pickup_id);
            self.change_player(player_id, &|player| {
                player.increase_scale();
                player.add_points(30);
            });
        }
    }

    pub fn update(&mut self) -> Option<JsonValue> {
        let time = self.clock.now();

        self.prune_inactive_players(time);
        self.collide_pickups(time);

        if self.pickups.len() < 100 {
            self.add_random_pickup(time);
        }

        // TODO flush here
        None
    }

    pub fn update_player(&mut self, id : u64, pos : &V2, vel: &V2, time: u64) {

        self.change_player(id, &|p| {
            p.update(time, pos, vel);
        });

        self.collide_pickups(time);
        // TODO flush here
    }
}

pub fn make_gamestate() -> Arc<Mutex<GameState>> {
    Arc::new(Mutex::new(GameState::new()))
}
