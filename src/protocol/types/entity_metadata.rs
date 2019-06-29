use super::slot::Slot;
use crate::protocol::types::chat::Chat;

pub enum EntityMetadataValue {
  Byte(i8),
  VarInt(i32),
  Float(f32),
  StringType(String),
  Chat(Chat),
  OptChat(Option<Chat>),
  Slot(Slot)
}

pub struct EntityMetadata {
  pub index: u8,
  pub type_: i32,
  pub value: EntityMetadataValue,
}