use std::collections::HashMap;
use json::JsonValue;
use rand;
use pickup::{Pickup, PickupType};
use clock;
use v2::{V2};

use messages::{Message, Payload};

use player::{Player};
use std::sync::mpsc::{Receiver,channel, Sender};

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

// use connectable::Connectable;

// impl Connectable<Message> for GameState {
//     fn connect(&mut self, tx : Sender<Message> ) -> Sender<Message> {
//         panic!("kjskjsa")
//     }
// }


////////////////////////////////////////////////////////////////////////////////
pub struct GameState {
    pub clock: clock::Clock,
    pub players: HashMap<u64, Player>,
    pub pickups : HashMap<u64, Pickup>,
    new_next_id : u64,
    time: u64,

    tx_to_server: Sender<Message>,
    rx_from_server: Receiver<Message>,
    tx_to_me: Sender<Message>,
}

impl GameState {
    
    pub fn new(tx_to_server: Sender<Message>) -> Self {
        let (tx_to_me, rx_from_server) = channel();
        let mut ret = Self {
            players: HashMap::new(),
            pickups: HashMap::new(),
            new_next_id : 0,
            clock: clock::Clock::new(),
            time: 0,
            tx_to_server, rx_from_server, tx_to_me,
        };

        let time = ret.clock.now();
        ret.time = time;

        for _ in 0..100 {
            ret.add_random_pickup();
        }
        ret
    }

    pub fn handle_message(&mut self, msg : Message) {
        use messages::Payload::*;

        match msg.data {
            Hello(_hello_payload) => {
            }

            Nothing => {},
            Unknown(_) => {},
            PlayerInfo(_) => {},
            State(_) => {},
            Delete(_) => {},
            Ping => {},
            Pong(_) => {},
            PickupInfo(_) => {},
            PlayerUpdate(_) => {},
        };
    }

    pub fn get_sender(&self) -> Sender<Message> {
        self.tx_to_me.clone()
    }

    pub fn broadcast(&mut self, data : Payload) {
        self.send(0, data)
    }

    pub fn send(&mut self, id : u64, data : Payload ) {
        let _message = Message::new(data, id, 0);
    }

    pub fn get_uuid(&mut self) -> u64 {
        let ret = self.new_next_id;
        self.new_next_id = self.new_next_id + 1;
        ret
    }

    pub fn add_random_pickup(&mut self) -> u64 {
        let p = mk_random_pickup(self.time);
        self.add_pickup(p)
    }

    pub fn add_pickup(&mut self, pickup : Pickup) -> u64 {
        let mut pickup = pickup.clone();
        let id = self.get_uuid();
        pickup.uuid = id;
        self.broadcast(Payload::PickupInfo((&pickup).into()));
        self.pickups.insert(id, pickup);
        id
    }

    pub fn remove_pickup(&mut self, id : u64) {
        if let Some(pickup) = self.pickups.get(&id) {
            info!("removing pickup: {}, type: {:?}", id, pickup.pickup_type );
        } else {
            warn!("failure to remove pickup {}", id );
        }

        let _ = self.pickups.remove(&id);
        self.broadcast(Payload::Delete(id));
    }

    pub fn add_session(&mut self, _connection_id : u64) {
    }

    pub fn add_player(&mut self, session_id : u64,  name : String, pos : V2, time : u64) -> u64 {
        let uuid = self.get_uuid();
        let player = Player::new(session_id, uuid, time, &name, pos.clone());
        self.broadcast(Payload::PlayerInfo((&player).into()));
        self.players.insert(uuid, player);
        uuid
    }

    pub fn remove_player(&mut self, id : u64) {
        info!("deleting player {}", id);
        self.players.remove(&id).unwrap();
        self.broadcast(Payload::Delete(id));
    }


    pub fn change_player(&mut self, id : u64, func : &Fn(&mut Player) ) {
        if let Some(p) = self.players.get_mut(&id) { 
            func(p);
        }

        if let Some(_p) = self.players.get(&id) {
            // TODO
            // self.broadcast(Payload::PlayerInfo(p.into()));
        } 
    }
}

impl GameState {


    fn prune_inactive_players(&mut self) {
        let time = self.time;

        let to_kill : Vec<u64> = self.players.iter().filter(|&(_,p)| {
            let since_last = p.since_last_update(time);
            since_last > 3.0
        }).map(|(k,_p)| *k).collect();

        for id in to_kill {
            self.remove_player(id);
        }
    }

    fn collide_pickups(&mut self) {

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
        self.time = self.clock.now();

        self.prune_inactive_players();
        self.collide_pickups();

        if self.pickups.len() < 100 {
            self.add_random_pickup();
        }

        None
    }

    pub fn update_player(&mut self, id : u64, pos : V2, vel: V2, time: u64) {
        self.change_player(id, &|p| {
            p.update(time, pos, vel);
        });
    }
}

