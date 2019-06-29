use super::chat::Chat;

pub struct TMCreate {
  pub display_name: Chat,
  pub friendly_flags: i8,
  pub name_visibility: String,
  pub collision: String,
  pub formatting: i32,
  pub prefix: Chat,
  pub suffix: Chat,
  pub entities: Vec<String>
}

pub struct TMUpdate {
  pub display_name: Chat,
  pub friendly_flags: i8,
  pub name_visibility: String,
  pub collision: String,
  pub formatting: i32,
  pub prefix: Chat,
  pub suffix: Chat,
}

pub struct TMEntities {
  pub entities: Vec<String>,
}

pub enum TeamMode {
  Create(TMCreate),
  Remove,
  UpdateInfo(TMUpdate),
  AddPlayers(TMEntities),
  RemovePlayers(TMEntities),
}