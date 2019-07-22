define_packet!(SpawnGlobalEntity, {
  entity_id: varint,
  type_: i8,
  x: f64,
  y: f64,
  z: f64,
});