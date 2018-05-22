use errors::Errors;
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

pub fn to_u64(json : &JsonValue, key : &str) -> Result<u64, Errors> {
    let j = get_key(json, key)?;

    if let Some(ret) = j.as_u64() {
        Ok(ret)
    } else {
        Err(Errors::Parsing(j.to_string()))
    }
}

pub fn to_f64(json : &JsonValue, key : &str) -> Result<f64, Errors> {
    let j = json[key].clone();
    if j.is_null() {
        Err(Errors::Missing(key.to_string()))
    } else if let Some(ret) = j.as_f64() {
        Ok(ret)
    } else {
        Err(Errors::Parsing(j.to_string()))
    }
}

pub fn to_v2(json : &JsonValue, key : &str) -> Result<v2::V2, Errors> {
    let j = get_key(json, key)?;
    let x = to_f64(j, "x")?;
    let y = to_f64(j, "y")?;
    Ok(v2::V2::new(x,y))
}


