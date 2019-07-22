define_packet!(EntityEquipment, {
  id: varint,
  slot: i8,
  item: slot,
});