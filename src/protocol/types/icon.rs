use super::chat::Chat;

pub struct Icon {
  pub type_: i32,
  pub x: i8,
  pub z: i8,
  pub direction: u8,
  pub display_name: Option<Chat>,
}