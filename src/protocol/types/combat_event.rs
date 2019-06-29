use super::chat::Chat;

pub enum CombatEvents {
  EnterCombat,
  EndCombat(i32, i32),
  EntityDead(i32, i32, Chat),
}