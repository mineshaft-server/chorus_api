use crate::type_definitions::direction::Direction;
use crate::protocol::types::position::Position;

pub struct SpawnPainting {
  pub entity_id: i32,
  pub uuid: u128,
  pub motive: i32,
  pub location: Position,
  pub direction: Direction,
}