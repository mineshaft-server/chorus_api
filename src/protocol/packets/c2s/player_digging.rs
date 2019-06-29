use crate::protocol::types::position::Position;

pub struct PlayerDigging {
  pub status: i32,
  pub location: Position,
  pub face: i8,
}