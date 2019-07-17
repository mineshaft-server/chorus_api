define_packet!(UseEntity, {
  target: varint,
  type_: varint,
  x: depends(type_ = 2) f32,
  y: depends(type_ = 2) f32,
  z: depends(type_ = 2) f32,
  hand: depends(type_ != 1) varint
});