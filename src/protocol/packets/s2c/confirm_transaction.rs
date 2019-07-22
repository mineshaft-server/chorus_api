define_packet!(ConfirmTransaction, {
  window_id: i8,
  action: i16,
  accepted: bool,
});