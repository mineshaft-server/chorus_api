use crate::protocol::types::position::Position;

pub struct Effect {
  pub id: i32,
  pub location: Position,
  pub data: i32,
  pub relative_volume: bool,
}