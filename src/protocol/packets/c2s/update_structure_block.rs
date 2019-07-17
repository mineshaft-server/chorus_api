define_packet!(UpdateStructureBlock, {
  location: position,
  action: varint,
  mode: varint,
  name: string,
  offset_x: i8,
  offset_y: i8,
  offset_z: i8,
  size_x: i8,
  size_y: i8,
  size_z: i8,
  mirror: varint,
  rotation: varint,
  metadata: string,
  integrity: f32,
  seed: varlong,
  flags: i8
});