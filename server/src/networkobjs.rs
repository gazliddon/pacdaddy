use obj::Obj;
use json::{JsonValue};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NetworkObjs {
    pub objs : HashMap<u64, Obj>,
    id  : u64,
    dirty: bool,
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
        let id = 0;
        let dirty = false;
        Self { objs, id, dirty }
    }

    pub fn add(&mut self, obj : Obj) -> u64 {
        let mut obj = obj.clone();
        let id = self.id;

        obj.id = id;
        self.id = id + 1;

        self.objs.insert(id, obj);
        self.set_dirty();
        id
    }

    pub fn get_mut<'a>(&'a mut self, _id : u64) -> Option<&'a mut Obj> {
        self.objs.get_mut(&_id)
    }

    pub fn remove(&mut self, id : u64) {
        if let Some(obj) = self.objs.get(&id) {
            info!("removing obj: {}, type: {:?}", id, obj.obj_type );
        } else {
            warn!("failure to remove obj {}", id );
        }

        self.set_dirty();
        self.objs.remove(&id).unwrap();
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }
}
