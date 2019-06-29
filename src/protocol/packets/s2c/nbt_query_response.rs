use crate::protocol::types::nbt::NBT;

pub struct NBTQueryResponse {
  pub transaction_id: i32,
  pub nbt: NBT,
}