use super::chat::Chat;
use super::slot::Slot;
use crate::util::identifier::Identifier;

pub struct AdvancmentDisplay {
  pub title: Chat,
  pub description: Chat,
  pub icon: Slot,
  pub frame: i32,
  pub flags: i32,
  pub background: Option<Identifier>,
  pub x: f32,
  pub y: f32,
}

pub struct AdvancementCriteria {
  pub achieved: bool,
  pub achieved_at: Option<i64>,
}

pub struct AdvancementProgress {
  pub criteria: Vec<(Identifier, AdvancementCriteria)>,
}

pub struct Advancement {
  pub parent: Option<Identifier>,
  pub display: Option<AdvancmentDisplay>,
  pub criteria: Vec<(Identifier, i8)>,
  pub requirements: Vec<(i32, Vec<String>)>,
}