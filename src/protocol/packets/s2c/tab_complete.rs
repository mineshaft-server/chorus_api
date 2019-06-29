use crate::protocol::types::tab_complete_match::TabCompleteMatch;

pub struct TabComplete {
  pub id: i32,
  pub start: i32,
  pub length: i32,
  pub matches: Vec<TabCompleteMatch>,
}