define_packet!(Handshake, {
  protocol: varint,
  address: string,
  port: u16,
  state: varint
});