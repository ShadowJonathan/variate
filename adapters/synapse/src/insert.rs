use variate_lib::{
    traits::{
        PduInserter, ReadReceiptsInserter, RoomInserter, ToDeviceMessageInserter, UserInserter,
    },
    Inserter,
};

use crate::DatabasePuck;

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
