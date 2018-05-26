use messages::{Message, Payload};
use errors::Errors;
use json;
use json::JsonValue;
use v2;

fn get_key<'a>(json : &'a JsonValue, key : &str) -> Result<&'a JsonValue, Errors> {
    let j = &json[key];
    if j.is_null() {
        Err(Errors::Missing(key.to_string())) 
    } else {
        Ok(j)
    }
}

fn to_u64(json : &JsonValue, key : &str) -> Result<u64, Errors> {
    let j = get_key(json, key)?;

    if let Some(ret) = j.as_u64() {
        Ok(ret)
    } else {
        Err(Errors::Parsing(j.to_string()))
    }
}

fn to_f64(json : &JsonValue, key : &str) -> Result<f64, Errors> {
    let j = json[key].clone();
    if j.is_null() {
        Err(Errors::Missing(key.to_string()))
    } else if let Some(ret) = j.as_f64() {
        Ok(ret)
    } else {
        Err(Errors::Parsing(j.to_string()))
    }
}

fn to_v2(json : &JsonValue, key : &str) -> Result<v2::V2, Errors> {
    let j = get_key(json, key)?;
    let x = to_f64(j, "x")?;
    let y = to_f64(j, "y")?;
    Ok(v2::V2::new(x,y))
}


impl Payload {
    fn from_raw(msg_str : &str, _j : &json::JsonValue) -> Result<Payload, Errors> {
        let _data = match msg_str {
            "hello" => {
                Payload::Unknown(msg_str.to_string())
            }

            "playerInfo" => {
                Payload::Unknown(msg_str.to_string())
            }

            "pong" => {
                Payload::Unknown(msg_str.to_string())
            }

            "nothing" | "raw" | "ping" => {
                Payload::Unknown(msg_str.to_string())
            }

            _ => {
                Payload::Unknown(msg_str.to_string())
            }
        };
        panic!("")
    }
}

impl Message {
    pub fn from_str(text: &str) -> Result<Self, Errors> {
        let parsed = json::parse(text)?;

        let time = to_u64(&parsed, "time")?;
        let id = to_u64(&parsed, "id")?;
        let msg = parsed["msg"].to_string();

        let data = Payload::from_raw(&msg, &parsed["data"])?;

        let ret = Message {
            msg, time, id, data, 
        };

        Ok(ret)
    }
}
