use json::JsonValue;
use json;
use errors::Errors;
use jsonparse::{to_u64};

pub struct MsgHdr {
    msg : String,
    time : u64,
    id   : u64,
    pub data : JsonValue,
    pub original : String,
}

impl MsgHdr {
    pub fn from_str(text: &str) -> Result<Self, Errors> {
        let parsed = json::parse(text)?;

        let time = to_u64(&parsed, "time")?;
        let id = to_u64(&parsed, "id")?;

        let msg = parsed["msg"].to_string();
        let data = parsed["data"].clone();

        let ret = MsgHdr {
            msg, time, id, data,
            original: text.to_string(),
        };

        Ok(ret)
    }

    pub fn get_type<'a>(&'a self) -> &'a str {
        &self.msg
    }

    pub fn get_time(&self) -> u64 {
        self.time
    }

    pub fn get_client_id(&self) -> u64 {
        self.id
    }
}
