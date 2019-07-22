define_packet!(SpawnObject, {
  entity_id: varint,
  uuid: uuid,
  type_: i8,
  x: f64,
  y: f64,
  z: f64,
  pitch: i8,
  yaw: i8,
  data: i32,
  velocity_x: i16,
  velocity_y: i16,
  velocity_z: i16
});