use crate::protocol::types::multiblock_change_record::MultiblockChangeRecord;

pub struct MultiblockChange {
  pub chunk_x: i32,
  pub chunk_y: i32,
  pub records: Vec<MultiblockChangeRecord>,
}