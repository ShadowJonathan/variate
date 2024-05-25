use std::{cell::RefMut, collections::HashMap};

use postgres::{fallible_iterator::FallibleIterator, Client, RowIter};
use postgres_types::{FromSql, Json, ToSql};
use serde::Deserialize;
use serde_json::{map::Map, value::Value};
use variate_lib::{
    traits::{
        PduExtractor, ReadReceiptsExtractor, RoomExtractor, ToDeviceMessageExtractor, UserExtractor,
    },
    types::{BIter, DeviceId, EventId, Pdu, RoomReadReceipts, RoomId, ToDeviceMessage, UserId},
    Extractor,
};

use crate::DatabasePuck;

pub struct SynapseExtractor {
    puck: DatabasePuck,
}

impl SynapseExtractor {
    pub(crate) fn new(puck: DatabasePuck) -> Self {
        Self { puck }
    }
}

impl Extractor for SynapseExtractor {
    fn pdu_e(&mut self) -> &mut dyn PduExtractor {
        self
    }

    fn rr_e(&mut self) -> &mut dyn ReadReceiptsExtractor {
        self
    }

    fn room_e(&mut self) -> &mut dyn RoomExtractor {
        self
    }

    fn usr_e(&mut self) -> &mut dyn UserExtractor {
        self
    }

    fn td_e(&mut self) -> &mut dyn ToDeviceMessageExtractor {
        self
    }
}

fn empty() -> std::iter::Empty<bool> {
    std::iter::empty::<bool>()
}

impl RoomExtractor for SynapseExtractor {
    fn all_known_ids(&mut self) -> Vec<RoomId> {
        let mut it = self.puck.pg.query_raw("SELECT room_id FROM rooms", empty()).unwrap();

        std::iter::from_fn(move || it.next().ok().flatten().map(|r| r.get("room_id"))).collect()
    }
}

impl UserExtractor for SynapseExtractor {
    fn all_local_ids(&mut self) -> Vec<UserId> {
        let mut it = self.puck.pg.query_raw("SELECT name FROM users", empty()).unwrap();

        std::iter::from_fn(move || it.next().ok().flatten().map(|r| r.get("name"))).collect()
    }
}

impl ToDeviceMessageExtractor for SynapseExtractor {
    fn all(&mut self) -> Vec<ToDeviceMessage> {
        let mut messages = vec![];
        let mut client = &mut self.puck.pg;

        if false {
            let mut it = client
                .query_raw("SELECT user_id, device_id, message_json FROM device_inbox", empty())
                .unwrap();

            #[derive(Deserialize)]
            struct MessageJson {
                pub content: Value,
                pub r#type: String,
                pub sender: String,
            }

            while let Some(row) = it.next().unwrap() {
                let user_id: UserId = row.get("user_id");
                let device_id: DeviceId = row.get("device_id");
                let message_json: String = row.get("message_json");
                let message = serde_json::from_str::<MessageJson>(&message_json).unwrap();

                messages.push(ToDeviceMessage {
                    user: user_id,
                    device: device_id,
                    sender: message.sender,
                    r#type: message.r#type,
                    message: message.content,
                })
            }
        }

        {
            let mut it = client
                .query_raw("SELECT messages_json FROM device_federation_outbox", empty())
                .unwrap();

            #[derive(Deserialize)]
            struct MessagesJson {
                pub messages: HashMap<UserId, HashMap<DeviceId, Value>>,
                pub sender: UserId,
                pub r#type: String,
            }

            while let Some(row) = it.next().unwrap() {
                let messages_json: String = row.get("messages_json");
                let messages_data = serde_json::from_str::<MessagesJson>(&messages_json).unwrap();

                for (user, devices) in messages_data.messages {
                    for (device, message) in devices {
                        messages.push(ToDeviceMessage {
                            user: user.clone(),
                            device,
                            sender: messages_data.sender.clone(),
                            r#type: messages_data.r#type.clone(),
                            message,
                        })
                    }
                }
            }
        }

        messages
    }
}

impl PduExtractor for SynapseExtractor {
    fn for_room<'a>(&'a mut self, room: &RoomId) -> BIter<'a, Pdu> {
        struct TryThis<'a> {
            iter: RowIter<'a>,
        }

        impl<'a, 'b> Iterator for TryThis<'a> {
            type Item = Pdu;

            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next().ok().flatten().map(|r| {
                    let id = r.get::<_, String>("event_id");
                    let version: i32 = r.get("format_version");
                    let json: String = r.get("json");
                    let value: Map<String, Value> = serde_json::from_str(&json).unwrap();

                    // a bunch of sanity checks
                    assert!(value.contains_key("type"), "event did not have type");
                    if version == 1 {
                        assert!(
                            value.contains_key("event_id"),
                            "event version 1 did not have event_id: {} ({}), {:#?}",
                            id,
                            version,
                            value
                        );
                    } else {
                        assert!(
                            !value.contains_key("event_id"),
                            "event version !1 did have event_id: {} ({}), {:#?}",
                            id,
                            version,
                            value
                        );
                    }
                    assert!(value.contains_key("content"));
                    assert!(value.contains_key("room_id"));

                    Pdu { id, value }
                })
            }
        }

        let iter: RowIter<'a> = self
            .puck
            .pg
            .query_raw(
                // todo remove 1 = 1
                "SELECT event_id, json, format_version FROM event_json WHERE room_id = $1 OR 1 = 1",
                &[room],
            )
            .unwrap();

        Box::new(TryThis { iter })
    }
}

impl ReadReceiptsExtractor for SynapseExtractor {
    fn for_room(&mut self, room: &RoomId) -> RoomReadReceipts {
        todo!()
    }
}
