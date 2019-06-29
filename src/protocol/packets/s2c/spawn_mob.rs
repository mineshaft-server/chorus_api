use crate::protocol::types::entity_metadata::EntityMetadata;

pub struct SpawnMob {
  pub entity_id: i32,
  pub uuid: u128,
  pub type_: i32,
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub yaw: i8,
  pub pitch: i8,
  pub head_pitch: i8,
  pub velocity_x: i16,
  pub velocity_y: i16,
  pub velocity_z: i16,
  pub metadata: EntityMetadata,
}