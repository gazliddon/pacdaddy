use cgmath::{Vector2};
pub type V2 = Vector2<f64>;

use json::{JsonValue};

#[derive(Debug, Clone, PartialEq)]
pub enum ObjType {
    Pickup,
    Player,
}

#[derive(Debug, Clone)]
pub struct Obj {
    pub id : u64,
    pub pos : V2,
    pub vel : V2,
    pub time : u64,
    pub kind: String,
    pub alive: bool,
    pub obj_type : ObjType,
    pub scale : f64,
}

impl<'a> From<&'a ObjType> for JsonValue {
    fn from(v : &'a ObjType) -> JsonValue {
        format!("{:?}", v).into()
    }
}

pub struct MyV2(pub V2);


impl<'a> From<&'a MyV2> for JsonValue {
    fn from(v : &'a MyV2) -> JsonValue {
        object!{
            "x" => v.0.x,
            "y" => v.0.y,
        }
    }
}

impl<'a> From<&'a Obj> for JsonValue {
    fn from(o : &'a Obj) -> JsonValue {
        object!{
            "id" => o.id,
            "pos" => &MyV2(o.pos),
            "vel" => &MyV2(o.vel),
            "kind" => o.kind.clone(),
            "time" => o.time,
            "alive" => o.alive,
            "obj_type" => &o.obj_type,
            "scale" => o.scale,
        }
    }
}

impl Obj {
    pub fn new(obj_type : ObjType, id : u64, pos : V2, vel : V2, time : u64, kind : &str) -> Obj {
        Obj {obj_type, id, pos, vel, time, kind: kind.to_string(), alive: true, scale: 0.0 }
    }

    pub fn update(&mut self) {
        self.pos = self.pos + self.vel;
    }
}
