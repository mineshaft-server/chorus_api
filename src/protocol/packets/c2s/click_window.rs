define_packet!(ClickWindow, {
  window_id: u8,
  slot: i16,
  button: i8,
  action: i16,
  mode: varint,
  clicked_item: slot
});