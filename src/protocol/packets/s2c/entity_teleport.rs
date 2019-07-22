define_packet!(EntityTeleport, {
  id: varint,
  x: f64,
  y: f64,
  z: f64,
  yaw: i8,
  pitch: i8,
  on_ground: bool,
});