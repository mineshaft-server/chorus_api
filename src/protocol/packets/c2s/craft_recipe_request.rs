define_packet!(CraftRecipeRequest, {
  window_id: i8,
  recipe: identifier,
  make_all: bool
});