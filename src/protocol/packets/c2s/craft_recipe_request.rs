use crate::util::identifier::Identifier;

pub struct CraftRecipeRequest {
  pub window_id: i8,
  pub recipe: Identifier,
  pub make_all: bool,
}