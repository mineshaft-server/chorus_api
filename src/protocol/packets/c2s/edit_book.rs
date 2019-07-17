define_packet!(EditBook, {
  book: slot,
  signing: bool,
  hand: varint
});