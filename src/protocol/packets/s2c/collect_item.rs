define_packet!(CollectItem, {
  collected_id: varint,
  collector_id: varint,
  pickup_item_count: varint,
});