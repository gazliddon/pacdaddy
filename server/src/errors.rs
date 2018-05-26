use ws;
use json;
use std::sync::mpsc::TryRecvError;

pub enum Errors {
    Json(json::JsonError),
    Missing(String),
    Parsing(String),
    Sockets(ws::Error),
    ChannelEmpty,
    ChannelDisconnected,
    UnhandledMessage,
}


impl From<json::Error> for Errors {
    fn from(e : json::JsonError) -> Errors {
        Errors::Json(e)
    }
}

impl From<ws::Error> for Errors {
    fn from(e : ws::Error) -> Errors {
        Errors::Sockets(e)
    }
}

impl From<TryRecvError> for Errors {
    fn from(e : TryRecvError) -> Errors {
        match e {
            TryRecvError::Empty => Errors::ChannelEmpty,
            TryRecvError::Disconnected => Errors::ChannelDisconnected,
        }
    }

}


