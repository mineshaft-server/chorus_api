use crate::protocol::types::chat::Chat;

pub enum BossBarAction {
  Add((Chat, f32, i32, i32,u8)),
  Remove,
  UpdateHealth(f32),
  UpdateTitle(Chat),
  UpdateStyle(i32, i32),
  UpdateFlags(u8)
}

pub struct BossBar {
  pub uuid: u128,
  pub action: BossBarAction,
}