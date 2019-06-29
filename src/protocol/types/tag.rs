use crate::util::identifier::Identifier;

pub struct Tag {
  pub name: Identifier,
  pub entries: Vec<i32>,
}