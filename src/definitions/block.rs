use crate::traits::Block;
use crate::utility::{
  Identifier,
  Transparency
};

pub struct BlockDef {
  pub id: Identifier,
  pub transparency: Transparency,
  pub blast_resistance: f64,
  pub hardness: f64,
  pub luminance: i8,
  pub flammable: bool,
  pub methods: Block,
}