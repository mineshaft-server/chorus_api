define_packet!(EntityRelativeMove, {
  entity_id: varint,
  delta_x: i16,
  delta_y: i16,
  delta_z: i16,
  on_ground: bool,
});