use crate::util::identifier::Identifier;

pub struct CraftRecipeResponse {
  pub window_id: i8,
  pub recipe: Identifier,
}