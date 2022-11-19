use super::types;

pub type BIter<T> = Box<dyn Iterator<Item = T>>;

// user -> room -> PDU -> RR

pub trait PduExtractor {
    fn for_room(&self, room: &types::RoomId) -> BIter<types::Pdu>;
}

pub trait PduInserter {
    fn add_to_room(&mut self, room: &types::RoomId, pdu: types::Pdu);
}

pub trait RoomExtractor {
    fn all_known_ids(&self) -> BIter<types::RoomId>;
}

pub trait RoomInserter {
    fn skeleton_room(&mut self, room: types::RoomId);
}

pub trait UserExtractor {
    fn all_local_ids(&self) -> BIter<types::UserId>;
}

pub trait UserInserter {
    fn skeleton_user(&mut self, id: types::UserId);
}

pub trait ReadReceiptsExtractor {
    fn for_room(&self, room: &types::RoomId) -> types::ReadReceipts;
}

pub trait ReadReceiptsInserter {
    fn into_room(&mut self, room: &types::RoomId, rr: types::ReadReceipts);
}

pub trait ToDeviceMessageExtractor {
    fn all(&self) -> BIter<types::ToDeviceMessage>;
}

pub trait ToDeviceMessageInserter {
    fn add(&mut self, message: types::ToDeviceMessage);
}
