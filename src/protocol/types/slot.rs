use super::nbt::NBT;

pub struct Slot {
  pub item_id: Option<i32>,
  pub item_count: Option<i32>,
  pub nbt: Option<NBT>,
}