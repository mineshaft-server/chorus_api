define_packet!(Effect, {
  id: varint,
  location: position,
  data: varint,
  relative_volume: bool,
});