use networkobjs::NetworkObjs;
use std::collections::HashMap;
use json::JsonValue;
use rand;
use obj::{Obj, V2, MyV2, ObjType};
use clock;

fn mk_random_vec() -> V2 {
    use rand::distributions::{IndependentSample, Range};

    let between = Range::new(0.0f64, 1000.0);
    let mut rng = rand::thread_rng();
    let x = between.ind_sample(&mut rng);
    let y = between.ind_sample(&mut rng);
    V2::new(x,y)
}

fn mk_random_pickup() -> Obj {
    use rand::Rng;

    let time = 0;

    let names = vec![
        "burger","coke", "pizza"
    ];

    let obj_type = rand::thread_rng().choose(&names);

    Obj::new(ObjType::Pickup, 0, mk_random_vec(), V2::new(0.0,0.0), time, obj_type.unwrap())
}


////////////////////////////////////////////////////////////////////////////////


#[derive(Clone)]
pub struct Player {
    id : u64,
    pos : V2,
    frame: u64,
    last_update: u64,
    scale : f64,
    score : u32,
}

impl Player {
    pub fn since_last_update(&self, now : u64) -> f64 {
        (now - self.last_update) as f64 / 1_000_000_000.0
    }
}

impl<'a> From<&'a Player> for JsonValue {
    fn from(o : &'a Player) -> JsonValue {
        object!{
            "id" => o.id,
            "pos" => &MyV2(o.pos),
            "scale" => o.scale,
            "score" => o.score,
        }
    }
}


#[derive(Clone)]
pub enum EventType {
    Deleted,
    Updated(Obj),
    Added(Obj),
}

#[derive(Clone)]
pub struct Event {
    pub ev_type : EventType,
    pub id : u64,
    pub time : u64,
}


#[derive(Clone)]
pub struct GameState {
    pub objs : NetworkObjs,
    pub players: HashMap<u64, Player>,
    next_id : u64,
    pub clock: clock::Clock,
    pub events : Vec<Event>,
}

impl<'a > From<&'a GameState> for JsonValue {
    fn from(o : &'a GameState) -> JsonValue {

        let players : Vec<&'a Player> = o.players.iter().map(|(_k,v)| v).collect();

        let jobjs = JsonValue::from(&o.objs);

        let ret = object!{
            "objs" => jobjs,
            "time" => 0,
            "players" => players
        };
        ret
    }
}

impl GameState {
    pub fn new() -> Self {
        let mut objs = NetworkObjs::new();

        for _ in 0..100 {
            let obj = mk_random_pickup();
            objs.add(obj);
        }

        Self {
            objs,
            players: HashMap::new(),
            next_id : 0,
            clock: clock::Clock::new(),
            events: vec![],
        }
    }

    pub fn add_obj(&mut self) -> u64 {
        self.objs.add(mk_random_pickup())
    }

    pub fn add_player(&mut self, name: &str, pos : &V2, time : u64) -> u64 {

        let mut obj = Obj::new(ObjType::Player, 0, *pos, V2::new(0.0, 0.0), time, "player");
        obj.name = Some(name.to_string());

        let id = self.objs.add(obj);

        let player = Player {
            id, pos: *pos, scale : 1.0,
            last_update : time,
            score: 0,
            frame: 0,
        };

        self.players.insert(id, player);

        id
    }

    fn prune_inactive_players(&mut self, time : u64) {

        let to_kill : Vec<(u64, u64)> = self.players.iter().filter(|&(_,p)| {
            let since_last = p.since_last_update(time);
            since_last > 3.0
        }).map(|(k,p)| (*k, p.id)).collect();

        for (k,ok) in to_kill {
            info!("deleting player {}", ok);
            self.players.remove(&k).unwrap();
            self.remove_obj(ok, time);
        }
    }

    pub fn remove_obj(&mut self, id : u64, time : u64) {
        self.objs.remove(id);
        self.events.push( Event {
            ev_type: EventType::Deleted,
            id, time
        });
    }

    pub fn update_obj(&mut self, id : u64, time : u64) {
        if let Some(obj) = self.objs.get(id) {
            self.events.push( Event {
                ev_type: EventType::Updated(obj.clone()),
                id, time
            });
        } else {
            warn!("Could not update obj id: {}", id);
        }
    }
    fn add_obj(&mut self, time : u64) {
        panic!("ksjakjskaj")
    }

    fn collide_pickups(&mut self, time : u64) -> usize {
        let mut pickup_hit : Vec<(u64, u64)> = vec![];

        let mut pickups_killed = 0;

        for (object_id, obj) in self.objs.objs.iter() {
            if obj.obj_type == ObjType::Pickup {
                for (player_id, player) in self.players.iter() {
                    use cgmath::prelude::*;
                    if player.pos.distance(obj.pos) < 15.0 {
                        pickup_hit.push((*player_id, *object_id));
                        break;
                    }
                }
            }
        }

        for &(player_id, pickup_id) in pickup_hit.iter() {
            pickups_killed = pickups_killed + 1;
            self.remove_obj(pickup_id, time);
            if let Some(player) = self.players.get_mut(&player_id) {
                player.scale = player.scale * 1.1;
            }
        }

        pickups_killed
    }

    pub fn update(&mut self) -> Option<JsonValue> {
        let time = self.clock.now();

        self.prune_inactive_players(time);

        let killed = self.collide_pickups(time);

        for _ in 0 .. killed {
            let obj = mk_random_pickup();
            self.objs.add(obj);
        }

        // update player objs
        
        for (pid, p) in self.players.iter() {
            if let Some(o) = self.objs.get_mut(*pid) {
                o.pos = p.pos.clone();
                o.scale = p.scale;
                o.dirty = true;
            }
        }

        None
    }


    pub fn get_updates(&mut self) -> Option<JsonValue> {
        panic!("")
    }

    pub fn update_player(&mut self, id : u64, pos : &V2, _time: u64) {
        if let Some(x) = self.players.get_mut(&id) {
            x.pos = pos.clone();
            x.last_update = self.clock.now();
        }
    }

}
