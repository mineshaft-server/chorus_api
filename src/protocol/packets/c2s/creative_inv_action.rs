use crate::protocol::types::slot::Slot;

pub struct CreativeInvAction {
    pub slot: i16,
    pub clicked_item: Slot,
}