define_packet!(UpdateBlockEntity, {
  position: position,
  action: u8,
  nbt: nbt,
});