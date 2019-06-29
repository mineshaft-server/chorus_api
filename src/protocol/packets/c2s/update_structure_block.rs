use crate::protocol::types::position::Position;

pub struct UpdateStructureBlock {
  pub location: Position,
  pub action: i32,
  pub mode: i32,
  pub name: String,
  pub offset_x: i8,
  pub offset_y: i8,
  pub offset_z: i8,
  pub size_x: i8,
  pub size_y: i8,
  pub size_z: i8,
  pub mirror: i32,
  pub rotation: i32,
  pub metadata: String,
  pub integrity: f32,
  pub seed: i64,
  pub flags: i8,
}