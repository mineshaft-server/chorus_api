define_packet!(EntityEffect, {
  id: varint,
  effect_id: varint,
  amplifier: i8,
  duration: varint,
  flags: i8,
});