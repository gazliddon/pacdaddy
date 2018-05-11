use obj::Obj;
use json::{JsonValue};

#[derive(Debug, Clone)]
pub struct NetworkObjs {
    pub objs : Vec<Obj>,
    id  : u64,
    dirty: bool,
}

impl<'a> From<&'a NetworkObjs> for JsonValue {
    fn from(o : &'a NetworkObjs) -> JsonValue {
        let objs : Vec<&'a Obj> = o.objs.iter().map(|v| v).collect();
        JsonValue::from(objs)
    }
}

impl NetworkObjs {
    pub fn new() -> Self {
        let objs = vec![];
        let id = 0;
        let dirty = false;
        Self { objs, id, dirty }
    }

    pub fn add(&mut self, obj : Obj) -> u64 {
        let mut obj = obj.clone();
        let id = self.id;

        obj.id = id;
        self.id = id + 1;
        self.objs.push(obj);
        self.set_dirty();

        id
    }

    pub fn remove(&mut self, id : u64) {
        let index = self.objs.iter().position(|x| x.id == id).unwrap();
        self.objs.remove(index);
        self.set_dirty();
    }


    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }
}
