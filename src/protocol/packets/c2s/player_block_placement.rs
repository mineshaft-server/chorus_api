use crate::protocol::types::position::Position;

pub struct PlayerBlockPlacement {
  pub location: Position,
  pub face: i32,
  pub hand: i32,
  pub cursor_x: f32,
  pub cursor_y: f32,
  pub cursor_z: f32,
}