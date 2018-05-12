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
pub struct GameState {
    pub objs : NetworkObjs,
    pub players: HashMap<u64, Player>,
    next_id : u64,
    pub clock: clock::Clock,
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
            clock: clock::Clock::new()
        }
    }

    pub fn add_obj(&mut self) -> u64 {
        self.objs.add(mk_random_pickup())
    }

    pub fn add_player(&mut self, pos : &V2, _time : u64) -> u64 {
        let obj = Obj::new(ObjType::Player, 0, *pos, V2::new(0.0, 0.0), 0, "player");

        let last_update = self.clock.time();

        let id = self.objs.add(obj);

        let player = Player {
            id, pos: *pos, scale : 1.0,
            last_update,
            score: 0,
        };

        self.players.insert(id, player);

        id
    }

    pub fn update(&mut self, _dt : f64) -> u64 {

        let now = self.clock.time();

        let to_kill : Vec<(u64, u64)> = self.players.iter().filter(|&(_,p)| {
            let since_last = p.since_last_update(now);
            since_last > 3.0

        }).map(|(k,p)| (*k, p.id)).collect();

        for (k,ok) in to_kill {
            info!("deleting player {}", ok);
            self.players.remove(&k).unwrap();
            self.objs.remove(ok);
        }

        for (id, p) in &mut self.players {
            use cgmath::prelude::*;
            let p_pos = p.pos;

            let hit : Vec<u64> = self.objs.objs.iter()
                .filter(|o| o.obj_type == ObjType::Pickup)
                .filter(|o| p_pos.distance(o.pos) < 10.0)
                .map(|o| o.id).collect();

            for pickup_id in hit {
                p.scale = p.scale * 1.1;
                self.objs.remove(pickup_id);
                let obj = mk_random_pickup();
                self.objs.add(obj);
            }

            for o in &mut self.objs.objs  {
                if o.id == *id {
                    o.scale = p.scale
                }
            }
        }

        0
    }

    pub fn update_player(&mut self, id : u64, pos : &V2, _time: u64) {

        if let Some(x) = self.players.get_mut(&id) {
            x.pos = pos.clone();
            x.last_update = self.clock.time();
        }

        for o in &mut self.objs.objs {
            if o.id == id {
                o.pos = pos.clone();
            }
        }
    }

}
