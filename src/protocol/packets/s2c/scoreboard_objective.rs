use crate::protocol::types::chat::Chat;

pub struct ScoreboardObjective {
  pub name: String,
  pub mode: i8,
  pub value: Option<Chat>,
  pub type_: Option<i32>,
}