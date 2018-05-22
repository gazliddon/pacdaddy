use std::sync::mpsc::{Sender};

pub trait Connectable<T>{
    fn connect(&mut self, tx : Sender<T> ) -> Sender<T>;
}
