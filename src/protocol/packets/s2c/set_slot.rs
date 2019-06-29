use crate::protocol::types::slot::Slot;

pub struct SetSlot {
  pub window_id: i8,
  pub slot: i16,
  pub data: Slot,
}