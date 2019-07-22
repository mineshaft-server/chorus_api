define_packet!(NBTQueryResponse, {
  transaction_id: varint,
  nbt: nbt,
});