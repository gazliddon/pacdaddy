use v2::V2;

#[derive(Debug, Clone, PartialEq)]
pub enum PickupType {
    Burger,
    Coke,
    Pizza,
}

#[derive(Debug, Clone)]
pub struct Pickup {
    pub id : u64,
    pub pos : V2,
    pub time : u64,
    pub pickup_type : PickupType,
}

impl Pickup {
    pub fn new(pickup_type : PickupType, id : u64, pos : V2, time : u64) -> Self {
        Self { pickup_type, id, pos, time }
    }
}
