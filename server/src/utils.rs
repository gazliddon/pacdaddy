use json;

pub fn mk_msg(msg : &str, j : json::JsonValue, time : u64) -> String {

    let mut res = object!{
        "msg" => msg,
        "time" => time,
    };

    res["data"] = j;

    res.to_string()
}
