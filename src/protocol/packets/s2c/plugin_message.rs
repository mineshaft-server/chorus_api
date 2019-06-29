use crate::util::identifier::Identifier;

pub struct PluginMessage {
  pub channel: Identifier,
  pub data: Vec<u8>,
}