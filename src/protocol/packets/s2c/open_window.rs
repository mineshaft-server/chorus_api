define_packet!(OpenWindow, {
  window_id: u8,
  window_type: string,
  window_title: chat,
  slot_count: u8,
  entity_id: depends(window_type = "EntityHorse") i32,
});