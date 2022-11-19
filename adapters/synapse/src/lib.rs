use std::cell::{Cell, RefCell};

use postgres::Client;
use variate_lib::{
    traits::{
        BIter, PduExtractor, PduInserter, ReadReceiptsExtractor, ReadReceiptsInserter,
        RoomExtractor, RoomInserter, ToDeviceMessageExtractor, ToDeviceMessageInserter,
        UserExtractor, UserInserter,
    },
    Extractor, Inserter,
};

pub fn make_extractor() -> Box<dyn Extractor> {
    todo!()
}

pub fn make_inserter() -> Box<dyn Inserter> {
    todo!()
}

struct DatabasePuck {
    client: RefCell<Client>,
}

pub struct SynapseExtractor {
    puck: DatabasePuck,
}

impl Extractor for SynapseExtractor {
    fn pdu_e(&self) -> &dyn PduExtractor {
        self
    }

    fn rr_e(&self) -> &dyn ReadReceiptsExtractor {
        self
    }

    fn room_e(&self) -> &dyn RoomExtractor {
        self
    }

    fn usr_e(&self) -> &dyn UserExtractor {
        self
    }

    fn td_e(&self) -> &dyn ToDeviceMessageExtractor {
        self
    }
}

impl PduExtractor for SynapseExtractor {
    fn for_room(&self, room: &variate_lib::types::RoomId) -> BIter<variate_lib::types::Pdu> {
        self.puck.client.borrow_mut()
    }
}

impl ReadReceiptsExtractor for SynapseExtractor {
    fn for_room(&self, room: &variate_lib::types::RoomId) -> variate_lib::types::ReadReceipts {
        todo!()
    }
}

impl RoomExtractor for SynapseExtractor {
    fn all_known_ids(&self) -> BIter<variate_lib::types::RoomId> {
        todo!()
    }
}

impl UserExtractor for SynapseExtractor {
    fn all_local_ids(&self) -> BIter<variate_lib::types::UserId> {
        todo!()
    }
}

impl ToDeviceMessageExtractor for SynapseExtractor {
    fn all(&self) -> BIter<variate_lib::types::ToDeviceMessage> {
        todo!()
    }
}

pub struct SynapseInserter {
    puck: DatabasePuck,
}

impl Inserter for SynapseInserter {
    fn pdu_i(&mut self) -> &mut dyn PduInserter {
        self
    }

    fn rr_i(&mut self) -> &mut dyn ReadReceiptsInserter {
        self
    }

    fn room_i(&mut self) -> &mut dyn RoomInserter {
        self
    }

    fn usr_i(&mut self) -> &mut dyn UserInserter {
        self
    }

    fn td_i(&mut self) -> &mut dyn ToDeviceMessageInserter {
        self
    }
}

impl PduInserter for SynapseInserter {
    fn add_to_room(&mut self, room: &variate_lib::types::RoomId, pdu: variate_lib::types::Pdu) {
        todo!()
    }
}

impl ReadReceiptsInserter for SynapseInserter {
    fn into_room(
        &mut self,
        room: &variate_lib::types::RoomId,
        rr: variate_lib::types::ReadReceipts,
    ) {
        todo!()
    }
}

impl RoomInserter for SynapseInserter {
    fn skeleton_room(&mut self, room: variate_lib::types::RoomId) {
        todo!()
    }
}

impl UserInserter for SynapseInserter {
    fn skeleton_user(&mut self, id: variate_lib::types::UserId) {
        todo!()
    }
}

impl ToDeviceMessageInserter for SynapseInserter {
    fn add(&mut self, message: variate_lib::types::ToDeviceMessage) {
        todo!()
    }
}
