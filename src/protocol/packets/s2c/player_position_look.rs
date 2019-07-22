define_packet!(PlayerPositionLook, {
  x: f64,
  y: f64,
  z: f64,
  yaw: f32,
  pitch: f32,
  flags: u8,
  teleport_id: varint,
});