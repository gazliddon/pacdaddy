use std::collections::HashMap;
use clock;
use v2::{V2};

use errors::Errors;

use gamestate::utils::{mk_random_pickup};
use messages::{Message, Payload, DeleteInfo, PlayerUpdateInfo, HelloInfo};
use gamestate::{Player, Pickup};
use std::sync::mpsc::{Receiver,channel, Sender};
// use gamestate::messages::*;

////////////////////////////////////////////////////////////////////////////////
pub struct GameState {
    pub clock: clock::Clock,
    pub players: HashMap<u64, Player>,
    pub pickups : HashMap<u64, Pickup>,
    pub new_next_id : u64,
    pub time: u64,

    pub tx_to_server: Sender<Message>,
    pub rx_from_server: Receiver<Message>,
    pub tx_to_me: Sender<Message>,
}

impl GameState {
    fn create_world(&mut self) {
        for _ in 0..100 {
            let id = self.get_uuid();
            let mut pickup = mk_random_pickup(self.time);
            pickup.uuid = id;
            self.pickups.insert(id, pickup);
        }
    }

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
        ret.create_world();
        ret
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
        self.broadcast(Payload::Delete(DeleteInfo {to_delete : id}));
    }

    pub fn add_player(&mut self, id : u64, client_time : u64, hello: HelloInfo) {

        let pos = V2::new(100.0, 100.0);

        use messages::PlayerJoinedInfo;

        let pjoined = PlayerJoinedInfo {
            uuid: id,
            pos: pos.clone()
        };

        self.send(id, Payload::PlayerJoined(pjoined));

        let gsinfo = GameStateInfo::from(&*self);
        self.send(id, Payload::State(gsinfo));

        let player = Player::new(id, client_time, client_time, &hello.name, pos.clone());

        self.broadcast(Payload::PlayerInfo((&player).into()));
        self.players.insert(id, player);
    }

    pub fn remove_player(&mut self, id : u64) {
        info!("deleting player {}", id);
        self.players.remove(&id).unwrap();
        self.broadcast(Payload::PlayerDelete(DeleteInfo{to_delete: id}));
    }

    pub fn change_player(&mut self, id : u64, func : &Fn(&mut Player) ) {
        if let Some(p) = self.players.get_mut(&id) { 
            func(p);
        }

        if let Some(p) = self.players.get(&id) {
            self.broadcast(Payload::PlayerInfo(p.into()));
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

    pub fn update_player(&mut self, id : u64, client_time : u64, pinfo : PlayerUpdateInfo) {
        let time = self.time;

        self.change_player(id, &|p| {
            p.update(time, client_time, pinfo.pos, pinfo.vel);
        });
    }

    pub fn update(&mut self) -> Result<(), Errors> {

        self.time = self.clock.now();

        let _messages_handled = self.process_messages()?;

        self.prune_inactive_players();
        self.collide_pickups();

        if self.pickups.len() < 100 {
            self.add_random_pickup();
        }
        Ok(())
    }
}


use messages::GameStateInfo;

impl<'a> From<&'a GameState> for GameStateInfo {
    fn from(gs : &'a GameState) -> GameStateInfo {
        use messages::PickupInfo;
        use messages::PlayerInfo;

        GameStateInfo {
            pickups: gs.pickups.iter().map(|(_,val)| PickupInfo::from(val)).collect(),
            players: gs.players.iter().map(|(_,val)| PlayerInfo::from(val)).collect(),
        }
    }
}
