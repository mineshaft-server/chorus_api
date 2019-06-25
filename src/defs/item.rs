use crate::utility::{
  Identifier,
  ItemType,
  ToolType
}

pub struct ItemDef {
  pub id: Identifier,
  pub item_type: ItemType,
  pub tool_type: ToolType,
  pub stack_size: u32
}