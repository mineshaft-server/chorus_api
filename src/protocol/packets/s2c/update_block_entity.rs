use crate::protocol::types::nbt::NBT;

pub struct UpdateBlockEntity {
  pub position: u64,
  pub action: u8,
  pub nbt: NBT,
}