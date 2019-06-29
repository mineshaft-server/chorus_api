use crate::protocol::types::tag::Tag;

pub struct Tags {
  pub blocks: Vec<Tag>,
  pub items: Vec<Tag>,
  pub fluids: Vec<Tag>,
}