define_packet!(UpdateCommandBlock, {
  location: position,
  command: string,
  mode: varint,
  flags: i8
});