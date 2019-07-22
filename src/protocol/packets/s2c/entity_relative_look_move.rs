define_packet!(EntityRelativeLookMove, {
  entity_id: varint,
  delta_x: i16,
  delta_y: i16,
  delta_z: i16,
  yaw: i8,
  pitch: i8,
  on_ground: bool,
});