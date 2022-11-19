use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::{map::Map, value::Value};

// unvetted user ID
#[repr(transparent)]
pub struct UserId(String);

#[repr(transparent)]
pub struct EventId(String);

#[repr(transparent)]
pub struct RoomId(String);

#[repr(transparent)]
pub struct DeviceId(String);

pub enum Untrusted<T> {
    Re(T),
    Un(Value),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pdu {
    #[serde(flatten)]
    value: Map<String, Value>,
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

pub struct ReadReceipts(HashMap<UserId, EventId>);

pub enum ToDeviceVariant {
    Incoming,
    Outgoing,
}

pub struct ToDeviceMessage {
    user: UserId,
    device: DeviceId,

    direction: ToDeviceVariant,

    message: Value,
}
