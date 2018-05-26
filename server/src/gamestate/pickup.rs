use v2::V2;
use messages::{PickupInfo};

#[derive(Debug, Clone, PartialEq)]
pub enum PickupType {
    Burger,
    Coke,
    Pizza,
}

#[derive(Debug, Clone)]
pub struct Pickup {
    pub uuid : u64,
    pub pos : V2,
    pub time : u64,
    pub pickup_type : PickupType,
}

impl Pickup {
    pub fn new(pickup_type : PickupType, uuid : u64, pos : V2, time : u64) -> Self {
        Self { pickup_type, uuid, pos, time }
    }
}

impl<'a> From<&'a Pickup> for PickupInfo {
    fn from(pickup : &'a Pickup) -> PickupInfo {
        PickupInfo {
            uuid : pickup.uuid,
            pos : pickup.pos.clone(),
            kind: format!("{:?}", &pickup.pickup_type)
        }
    }
}
