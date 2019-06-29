use crate::protocol::types::chat::Chat;

pub struct Disconnect {
  pub reason: Chat,
}