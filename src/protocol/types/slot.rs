use super::nbt::Tag;

pub struct Slot {
  pub item_count: i8,
  pub item_id: i32,
  pub nbt: Tag,
}