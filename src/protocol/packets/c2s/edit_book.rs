use crate::protocol::types::slot::Slot;

pub struct EditBook {
  pub book: Slot,
  pub signing: bool,
  pub hand: i32,
}