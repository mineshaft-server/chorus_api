use crate::protocol::types::nbt::Tag;

pub struct ChunkData {
  pub x: i32,
  pub z: i32,
  pub full_chunk: bool,
  pub bitmask: i32,
  pub data: Vec<u8>,
  pub block_entities: Vec<Tag>,
}