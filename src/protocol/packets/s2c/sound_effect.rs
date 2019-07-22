define_packet!(SoundEffect, {
  id: varint,
  category: varint,
  x: i32,
  y: i32,
  z: i32,
  volume: f32,
  pitch: f32,
});