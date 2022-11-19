pub mod traits;
pub mod types;

use traits::*;

pub trait Extractor {
    fn pdu_e(&self) -> &dyn PduExtractor;
    fn rr_e(&self) -> &dyn ReadReceiptsExtractor;
    fn room_e(&self) -> &dyn RoomExtractor;
    fn usr_e(&self) -> &dyn UserExtractor;
    fn td_e(&self) -> &dyn ToDeviceMessageExtractor;
}

pub trait Inserter {
    fn pdu_i(&mut self) -> &mut dyn PduInserter;
    fn rr_i(&mut self) -> &mut dyn ReadReceiptsInserter;
    fn room_i(&mut self) -> &mut dyn RoomInserter;
    fn usr_i(&mut self) -> &mut dyn UserInserter;
    fn td_i(&mut self) -> &mut dyn ToDeviceMessageInserter;
}
