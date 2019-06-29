use super::modifier_data::ModifierData;

pub struct EntityProperty {
  pub key: String,
  pub value: f64,
  pub modifiers: Vec<ModifierData>,
}