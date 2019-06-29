use crate::protocol::types::slot::Slot;

pub struct ClickWindow {
  pub window_id: u8,
  pub slot: i16,
  pub button: i8,
  pub action: i16,
  pub mode: i32,
  pub clicked_item: Slot,
}