use ws;
use json;

pub enum Errors {
    Json(json::JsonError),
    Missing(String),
    Parsing(String),
    Sockets(ws::Error)
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


