define_packet!(UpdateCommandBlockMinecart, {
  id: varint,
  command: string,
  track_output: bool
});