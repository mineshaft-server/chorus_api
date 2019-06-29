use crate::protocol::types::entity_metadata::EntityMetadata;

pub struct SpawnPlayer {
  pub entity_id: i32,
  pub uuid: u128,
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub yaw: i8,
  pub pitch: i8,
  pub metadata: EntityMetadata,
}