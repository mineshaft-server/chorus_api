use crate::util::identifier::Identifier;

pub struct NamedSoundEffect {
  pub name: Identifier,
  pub category: i32,
  pub x: i32,
  pub y: i32,
  pub z: i32,
  pub volume: f32,
  pub pitch: f32,
}