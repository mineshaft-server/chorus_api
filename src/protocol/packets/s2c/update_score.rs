pub struct UpdateScore {
  pub entity_name: String,
  pub action: i8,
  pub objective_name: String,
  pub value: Option<i32>,
}