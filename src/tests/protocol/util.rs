use crate::protocol::util;

#[test]
pub fn test_write_varint() {
  // Setup
  let mut subject: Vec<u8> = Vec::new();
  let mut value: i32 = 0x7F;

  // Test single varint byte write
  util::write_varint(&mut subject, &value);

  // Evaluate results
  assert_eq!(subject.len(), 1);
  assert_eq!(subject[0], 0x7F);

  // Reset
  subject.clear();

  // Test double varint byte write
  value = 128;
  util::write_varint(&mut subject, &value);

  // Evaluate results
  assert_eq!(subject.len(), 2);
  assert_eq!(subject[0], 0x80);
  assert_eq!(subject[1], 0x01);

  // Reset
  subject.clear();

  // Test positive max byte write
  value = 2147483647;
  util::write_varint(&mut subject, &value);

  // Evaluate results
  assert_eq!(subject.len(), 5);
  assert_eq!(subject[0], 0xFF);
  assert_eq!(subject[1], 0xFF);
  assert_eq!(subject[2], 0xFF);
  assert_eq!(subject[3], 0xFF);
  assert_eq!(subject[4], 0x07);

  // Reset
  subject.clear();

  // Test positive max byte write
  value = -2147483648;
  util::write_varint(&mut subject, &value);

  // Evaluate results
  assert_eq!(subject.len(), 5);
  assert_eq!(subject[0], 0x80);
  assert_eq!(subject[1], 0x80);
  assert_eq!(subject[2], 0x80);
  assert_eq!(subject[3], 0x80);
  assert_eq!(subject[4], 0x08);
}

#[test]
pub fn test_read_varint() {
  let mut data: Vec<u8> = Vec::new();
  data.push(0x7F);
  let mut value = util::read_varint(&mut data);
  assert_eq!(value, Some(127));
  assert_eq!(data.len(), 0);

  data.push(0x80);
  data.push(0x01);

  value = util::read_varint(&mut data);
  assert_eq!(value, Some(128));
  assert_eq!(data.len(), 0);


  data.push(0xFF);
  data.push(0xFF);
  data.push(0xFF);
  data.push(0xFF);
  data.push(0x07);

  value = util::read_varint(&mut data);
  assert_eq!(value, Some(2147483647));
  assert_eq!(data.len(), 0);

  data.push(0x80);
  data.push(0x80);
  data.push(0x80);
  data.push(0x80);
  data.push(0x08);

  value = util::read_varint(&mut data);
  assert_eq!(value, Some(-2147483648));
  assert_eq!(data.len(), 0);
}