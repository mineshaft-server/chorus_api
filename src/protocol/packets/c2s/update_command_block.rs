use crate::protocol::types::position::Position;

pub struct UpdateCommandBlock {
  pub location: Position,
  pub command: String,
  pub mode: i32,
  pub flags: i8,
}