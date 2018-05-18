use json;

pub fn mk_msg_json(msg : &str, time : u64, j : json::JsonValue) -> json::JsonValue {
    let mut res = object!{
        "msg" => msg,
        "time" => time,
    };
    res["data"] = j;
    res
}

pub fn mk_msg(msg : &str, time : u64, j : json::JsonValue) -> String {
    mk_msg_json(msg, time, j).to_string()
}
