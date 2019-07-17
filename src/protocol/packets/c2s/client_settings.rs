define_packet!(ClientSettings, {
  locale: string,
  view_distance: i8,
  chat_mode: varint,
  chat_colors: bool,
  skin_parts: u8,
  main_hand: varint
});