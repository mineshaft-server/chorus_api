define_packet!(SetCooldown, {
  item_id: varint,
  cooldown_ticks: varint,
});