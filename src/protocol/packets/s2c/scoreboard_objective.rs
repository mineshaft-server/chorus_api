define_packet!(ScoreboardObjective, {
  name: string,
  mode: i8,
  value: depends(mode != 1) chat,
  type_: depends(mode != 1) varint,
});