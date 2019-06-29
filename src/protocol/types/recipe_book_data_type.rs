use crate::util::identifier::Identifier;

pub enum RBDataType {
  DisplayedRecipe(Identifier),
  BookState(bool, bool, bool, bool),
}