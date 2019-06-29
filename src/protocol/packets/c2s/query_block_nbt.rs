use crate::protocol::types::position::Position;

pub struct QueryBlockNBT {
  pub transation_id: i32,
  pub location: Position,
}