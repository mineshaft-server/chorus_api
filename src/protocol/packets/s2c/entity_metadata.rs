use crate::protocol::types::entity_metadata::EntityMetadata as EntityMetadataType;

pub struct EntityMetadata {
  pub id: i32,
  pub metadata: EntityMetadataType,
}