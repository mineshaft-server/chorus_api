use crate::protocol::types::position::Position;

pub struct BlockBreakAnimation {
  pub entity_id: i32,
  pub position: Position,
  pub stage: i8,
}