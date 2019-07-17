define_packet!(QueryBlockNBT, {
  transation_id: varint,
  location: position
});