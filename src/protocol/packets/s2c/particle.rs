use crate::protocol::types::particle_data::ParticleData;

pub struct Particle {
  pub id: i32,
  pub long_distance: bool,
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub offset_x: f32,
  pub offset_y: f32,
  pub offset_z: f32,
  pub particle_data: f32,
  pub count: i32,
  pub data: ParticleData,
}