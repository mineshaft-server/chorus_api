use crate::util::identifier::Identifier;

pub struct UnlockRecipes {
  pub action: i32,
  pub craftbook_open: bool,
  pub craftbook_filter: bool,
  pub smeltbook_open: bool,
  pub smeltbook_filter: bool,
  pub recipes: Vec<Identifier>,
  pub opt_recipes: Option<Vec<Identifier>>,
}