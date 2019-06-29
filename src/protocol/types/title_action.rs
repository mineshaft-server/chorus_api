use super::chat::Chat;

pub enum TitleAction {
  SetTitle(Chat),
  SetSubtitle(Chat),
  SetActionBar(Chat),
  SetTimes(i32, i32, i32),
  Hide,
  Reset,
}