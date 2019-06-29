use super::slot::Slot;
use crate::util::identifier::Identifier;

pub struct Ingredient {
  pub items: Vec<Slot>,
}

pub struct RecipeShapeless {
  pub group: String,
  pub ingredients: Vec<Ingredient>,
  pub result: Slot,
}

pub struct RecipeShaped {
  pub width: i32,
  pub height: i32,
  pub group: String,
  pub ingredients: Vec<Ingredient>,
  pub result: Slot,
}

pub struct RecipeSmelting {
  pub group: String,
  pub ingredient: Ingredient,
  pub result: Slot,
  pub experience: f32,
  pub cook_time: i32,
}

pub enum RecipeData {
  Shapeless(RecipeShapeless),
  Shaped(RecipeShaped),
  Smelting(RecipeSmelting),
}

pub struct Recipe {
  pub id: Identifier,
  pub type_: String,
  pub data: Option<RecipeData>,
}