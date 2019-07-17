#[macro_use]
use crate::protocol::packets::internal::*;

define_packet!(AdvancementTab, {
  action: i32,
  id: identifier
});
