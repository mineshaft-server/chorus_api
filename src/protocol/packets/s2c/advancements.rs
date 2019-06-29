use crate::protocol::types::advancement::{
  Advancement,
  AdvancementProgress,
};
use crate::util::identifier::Identifier;

pub struct Advancements {
  pub reset: bool,
  pub mappings: Vec<(Identifier, Advancement)>,
  pub ids: Vec<Identifier>,
  pub progress: Vec<(Identifier, AdvancementProgress)>,
}