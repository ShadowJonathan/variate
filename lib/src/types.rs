use std::collections::HashMap;

use serde_json::{map::Map, value::Value};

pub type BIter<'a, T> = Box<dyn Iterator<Item = T> + 'a>;

// unvetted user ID
// #[repr(transparent)]
// #[derive(Debug, Hash, PartialEq, Eq, Deserialize, Serialize, ToSql)]
pub type UserId = String;

// #[repr(transparent)]
// #[derive(Debug)]
pub type EventId = String;

// #[repr(transparent)]
// #[derive(Debug)]
pub type RoomId = String;

// #[repr(transparent)]
// #[derive(Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
// pub struct DeviceId(pub String);
pub type DeviceId = String;

pub enum Untrusted<T> {
    Re(T),
    Un(Value),
}

#[derive(Debug)]
pub struct Pdu {
    pub id: EventId,
    pub value: Map<String, Value>,
}

impl Pdu {
    // None = invalid
    pub fn is_state(&self) -> Option<bool> {
        match self.value.get("state_key") {
            Some(Value::String(_)) => Some(true),
            Some(_) => None,
            None => Some(false),
        }
    }
}

#[derive(Debug)]
pub struct RoomReadReceipts(pub HashMap<UserId, Vec<EventId>>);

#[derive(Debug)]
pub struct ToDeviceMessage {
    pub user: UserId,
    pub device: DeviceId,
    pub sender: UserId,

    pub r#type: String,

    pub message: Value,
}
