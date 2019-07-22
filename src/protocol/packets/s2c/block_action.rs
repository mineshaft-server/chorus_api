
define_packet!(BlockAction, {
  location: position,
  action: u8,
  param: u8,
  block_type: varint,
});