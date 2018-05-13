use obj::Obj;
use json::{JsonValue};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NetworkObjs {
    pub objs : HashMap<u64, Obj>,
    next_id  : u64,
}

impl<'a> From<&'a NetworkObjs> for JsonValue {
    fn from(_o : &'a NetworkObjs) -> JsonValue {
        let objs : Vec<&'a Obj> = _o.objs.iter().map(|(_k,v)| v).collect();
        JsonValue::from(objs)
    }
}

impl NetworkObjs {
    pub fn new() -> Self {
        let objs = HashMap::new();
        let next_id = 0;
        Self { objs, next_id}
    }

    pub fn add(&mut self, obj : Obj) -> u64 {
        let mut obj = obj.clone();
        let id = self.next_id;

        obj.id = id;
        self.next_id = id + 1;

        self.objs.insert(id, obj);
        id
    }

    pub fn get_mut<'a>(&'a mut self, id : u64) -> Option<&'a mut Obj> {
        self.objs.get_mut(&id)
    }

    pub fn get<'a>(&'a self, id : u64) -> Option<&'a Obj> {
        self.objs.get(&id)
    }

    pub fn remove(&mut self, id : u64) {
        if let Some(obj) = self.objs.get(&id) {
            info!("removing obj: {}, type: {:?}", id, obj.obj_type );
        } else {
            warn!("failure to remove obj {}", id );
        }

        let _ = self.objs.remove(&id);
    }

}
