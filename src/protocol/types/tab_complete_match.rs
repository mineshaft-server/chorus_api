use crate::protocol::types::chat::Chat;

pub struct TabCompleteMatch {
  pub result: String,
  pub tooltip: Option<Chat>,
}