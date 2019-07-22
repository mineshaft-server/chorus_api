define_packet!(JoinGame, {
  entity_id: varint,
  gamemode: u8,
  dimension: varint,
  difficulty: u8,
  max_players: u8,
  level_type: string,
  reduced_debug: bool,
});