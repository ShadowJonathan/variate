use super::types;

// user -> room -> PDU -> RR

pub use extr::*;

mod extr {
    use crate::types::BIter;

    use super::types;

    pub trait PduExtractor {
        fn for_room(&mut self, room: &types::RoomId) -> BIter<types::Pdu>;
    }
    pub trait RoomExtractor {
        fn all_known_ids(&mut self) -> Vec<types::RoomId>;
    }

    pub trait UserExtractor {
        fn all_local_ids(&mut self) -> Vec<types::UserId>;
    }

    pub trait ReadReceiptsExtractor {
        fn for_room(&mut self, room: &types::RoomId) -> types::RoomReadReceipts;
    }
    pub trait ToDeviceMessageExtractor {
        /// Gets all to-device messages still to be forwarded.
        ///
        /// This includes outbox messages, and inbox messages.
        fn all(&mut self) -> Vec<types::ToDeviceMessage>;
    }
}

pub use ins::*;

mod ins {
    use super::types;

    pub trait PduInserter {
        fn add_to_room(&mut self, room: &types::RoomId, pdu: types::Pdu);
    }

    pub trait RoomInserter {
        fn skeleton_room(&mut self, room: types::RoomId);
    }

    pub trait UserInserter {
        fn skeleton_user(&mut self, id: types::UserId);
    }

    pub trait ReadReceiptsInserter {
        fn into_room(&mut self, room: &types::RoomId, rr: types::RoomReadReceipts);
    }

    pub trait ToDeviceMessageInserter {
        fn add(&mut self, message: types::ToDeviceMessage);
    }
}
