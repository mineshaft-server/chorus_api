define_packet!(Respawn, {
  dimension: varint,
  difficulty: u8,
  gamemode: u8,
  level: string,
});