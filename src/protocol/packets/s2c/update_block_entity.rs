use crate::protocol::types::nbt::Tag;

pub struct UpdateBlockEntity {
  pub position: u64,
  pub action: u8,
  pub nbt: Tag,
}