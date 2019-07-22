define_packet!(UpdateHealth, {
  health: f32,
  food: varint,
  saturation: f32,
});