use crate::protocol::types::position::Position;

pub struct BlockAction {
  pub location: Position,
  pub action: u8,
  pub param: u8,
  pub block_type: i32,
}