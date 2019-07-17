define_packet!(PlayerBlockPlacement, {
  location: position,
  face: varint,
  hand: varint,
  cursor_x: f32,
  cursor_y: f32,
  cursor_z: f32
});