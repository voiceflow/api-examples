use serde::Deserialize;
use serde_json::Map;
use std::iter::FromIterator;

const REQUEST: &str = "request";
const PAYLOAD: &str = "payload";
const TYPE: &str = "type";
mod types {
    pub const TEXT: &str = "text";
    pub const LAUNCH: &str = "launch";
}

pub trait Request {
    fn body(&self) -> serde_json::Value;
}

pub struct LaunchRequest;
impl Request for LaunchRequest {
    fn body(&self) -> serde_json::Value {
        Map::from_iter([(TYPE.into(), types::LAUNCH.into())]).into()
    }
}

pub struct TextRequest {
    payload: String,
}
impl Request for TextRequest {
    fn body(&self) -> serde_json::Value {
        let request = Map::from_iter([
            (TYPE.into(), types::TEXT.into()),
            (PAYLOAD.into(), self.payload.clone().into()),
        ])
        .into();

        Map::from_iter([(REQUEST.into(), request)]).into()
    }
}
impl TextRequest {
    pub fn new(payload: String) -> TextRequest {
        TextRequest { payload }
    }
}

#[derive(Deserialize, Debug)]
pub struct Trace {
    pub r#type: String,
    payload: Option<serde_json::Value>,
}
impl Trace {
    pub fn message(&self) -> serde_json::Result<&str> {
        use serde::de::Error;
        use serde_json::error::Error as JsonErr;
        use serde_json::Value;

        if let Some(ref payload) = self.payload {
            match payload {
                Value::String(ref message) => Ok(&message),
                Value::Object(ref map) => map
                    .get("message")
                    .and_then(Value::as_str)
                    .ok_or(JsonErr::custom("Missing message field in payload")),
                _ => Err(JsonErr::custom("Expected payload to be object")),
            }
        } else {
            Err(JsonErr::custom("Missing payload field"))
        }
    }
}
