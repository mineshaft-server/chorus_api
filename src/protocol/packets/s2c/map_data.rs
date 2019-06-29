use crate::protocol::types::icon::Icon;

pub struct MapData {
  pub map_id: i32,
  pub scale: i8,
  pub tracking_position: bool,
  pub icons: Vec<Icon>,
  pub columns: i8,
  pub rows: Option<i8>,
  pub x: Option<i8>,
  pub y: Option<i8>,
  pub data: Option<Vec<u8>>,
}