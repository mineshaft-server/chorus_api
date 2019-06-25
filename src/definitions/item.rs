use crate::utility::{
  Identifier,
  ItemType,
}

pub struct ItemDef {
  pub id: Identifier,
  pub item_type: ItemType,
  pub stack_size: u32
}