use crate::protocol::types::slot::Slot;

pub struct WindowItems {
  pub window_id: u8,
  pub slots: Vec<Slot>,
}