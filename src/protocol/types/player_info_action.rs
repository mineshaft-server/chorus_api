use super::chat::Chat;

pub struct PIAddProps {
  pub name: String,
  pub value: String,
  pub signature: Option<String>,
}

pub struct PIActionAdd {
  pub name: String,
  pub properties: Vec<PIAddProps>,
  pub gamemode: i32,
  pub ping: i32,
  pub display_name: Option<Chat>,
}

pub enum PlayerInfoAction {
  AddPlayer(PIActionAdd),
  UpdateGamemode(i32),
  UpdateLatency(i32),
  UpdateDisplayName(Option<Chat>),
  RemovePlayer,
}