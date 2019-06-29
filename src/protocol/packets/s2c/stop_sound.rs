use crate::util::identifier::Identifier;

pub struct StopSound {
  pub flags: i8,
  pub source: Option<i32>,
  pub sound: Option<Identifier>,
}