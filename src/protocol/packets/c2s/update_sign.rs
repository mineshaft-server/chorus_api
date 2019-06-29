use crate::protocol::types::position::Position;

pub struct UpdateSign {
  pub location: Position,
  pub lines: Vec<String>,
}