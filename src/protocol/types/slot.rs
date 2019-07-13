use super::nbt::NBT;

pub struct Slot {
  pub item_count: i8,
  pub item_id: Option<i32>,
  pub nbt: Option<NBT>,
}