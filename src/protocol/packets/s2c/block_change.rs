use crate::protocol::types::position::Position;

pub struct BlockChange {
  pub location: Position,
  pub block_id: i32,
}