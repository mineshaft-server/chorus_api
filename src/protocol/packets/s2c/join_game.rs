pub struct JoinGame {
  pub entity_id: i32,
  pub gamemode: u8,
  pub dimension: i32,
  pub difficulty: u8,
  pub max_players: u8,
  pub level_type: String,
  pub reduced_debug: bool,
}