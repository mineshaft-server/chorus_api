use crate::protocol::types::chat::Chat;

pub struct OpenWindow {
  pub window_id: u8,
  pub window_type: String,
  pub window_title: Chat,
  pub slot_count: u8,
  pub entity_id: Option<i32>,
}