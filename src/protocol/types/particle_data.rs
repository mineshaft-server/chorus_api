use super::slot::Slot;

pub struct Color {
  pub red: f32,
  pub green: f32,
  pub blue: f32,
  pub scale: f32,
}

pub enum ParticleData {
  None,
  Block(i32),
  Dust(Color),
  FallingDust(i32),
  Item(Slot),
}