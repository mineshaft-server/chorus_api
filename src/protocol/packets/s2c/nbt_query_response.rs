use crate::protocol::types::nbt::Tag;

pub struct NBTQueryResponse {
  pub transaction_id: i32,
  pub nbt: Tag,
}