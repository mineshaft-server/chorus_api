define_packet!(PlayerDigging, {
  status: varint,
  location: position,
  face: i8
});