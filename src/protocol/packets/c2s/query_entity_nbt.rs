define_packet!(QueryEntityNBT, {
  transaction_id: varint,
  entity_id: varint
});