use rand;
use v2::{V2};
use pickup::{Pickup, PickupType};

pub fn mk_random_vec() -> V2 {
    use rand::distributions::{IndependentSample, Range};
    let between = Range::new(0.0f64, 1000.0);
    let mut rng = rand::thread_rng();
    let x = between.ind_sample(&mut rng);
    let y = between.ind_sample(&mut rng);
    V2::new(x,y)
}

pub fn mk_random_pickup(time : u64) -> Pickup {
    use rand::Rng;

    let names = vec![
        PickupType::Coke,
        PickupType::Pizza,
        PickupType::Burger,
    ];

    let obj_type = rand::thread_rng().choose(&names).unwrap();

    Pickup::new(obj_type.clone(), 0, mk_random_vec(), time)
}
