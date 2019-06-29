use crate::protocol::types::slot::Slot;

pub struct EntityEquipment {
  pub id: i32,
  pub slot: i8,
  pub item: Slot,
}