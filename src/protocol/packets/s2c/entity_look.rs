define_packet!(EntityLook, {
  entity_id: varint,
  yaw: i8,
  pitch: i8,
  on_ground: bool,
});