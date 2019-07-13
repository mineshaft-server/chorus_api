extern crate json;

use super::types::{
  chat::Chat,
  position::Position,
  slot::Slot,
  nbt::NBT,
};

pub fn read_byte(data: &mut Vec<u8>) -> i8 {
  return data.remove(0) as i8;
}

pub fn write_byte(data: &mut Vec<u8>, value: &i8) {
  data.push(*value as u8);
}

#[inline(always)]
pub fn read_ubyte(data: &mut Vec<u8>) -> u8 {
  return data.remove(0);
}

#[inline(always)]
pub fn write_ubyte(data: &mut Vec<u8>, value: &u8) {
  data.push(*value);
}

pub fn read_short(data: &mut Vec<u8>) -> i16 {
  let bytes: Vec<u8> = data.drain(..2).collect();
  println!("{:?}", bytes);
  let _bytes: [u8; 2] = [bytes[0], bytes[1]];
  return i16::from_be_bytes(_bytes);
}

pub fn write_short(data: &mut Vec<u8>, value: &i16) {
  let bytes = value.to_be_bytes();
  data.reserve(2);
  write_ubyte(data, &bytes[0]);
  write_ubyte(data, &bytes[1]);
}

pub fn read_ushort(data: &mut Vec<u8>) -> u16 {
  let bytes: Vec<u8> = data.drain(..2).collect();
  println!("{:?}", bytes);
  let _bytes: [u8; 2] = [bytes[0], bytes[1]];
  return u16::from_be_bytes(_bytes);
}

pub fn write_ushort(data: &mut Vec<u8>, value: u16) {
  let bytes = value.to_be_bytes();
  data.reserve(2);
  write_ubyte(data, &bytes[0]);
  write_ubyte(data, &bytes[1]);
}

pub fn read_int(data: &mut Vec<u8>) -> i32 {
  let bytes: Vec<u8> = data.drain(..4).collect();
  let _bytes: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
  return i32::from_be_bytes(_bytes);
}

pub fn write_int(data: &mut Vec<u8>, value: &i32) {
  let bytes = value.to_be_bytes();
  data.reserve(4);
  write_ubyte(data, &bytes[0]);
  write_ubyte(data, &bytes[1]);
  write_ubyte(data, &bytes[2]);
  write_ubyte(data, &bytes[3]);
}

pub fn read_uint(data: &mut Vec<u8>) -> u32 {
  let bytes: Vec<u8> = data.drain(..4).collect();
  let _bytes: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
  return u32::from_be_bytes(_bytes);
}

pub fn write_uint(data: &mut Vec<u8>, value: &u32) {
  let bytes = value.to_be_bytes();
  data.reserve(4);
  write_ubyte(data, &bytes[0]);
  write_ubyte(data, &bytes[1]);
  write_ubyte(data, &bytes[2]);
  write_ubyte(data, &bytes[3]);
}

pub fn read_long(data: &mut Vec<u8>) -> i64 {
  let bytes: Vec<u8> = data.drain(..8).collect();
  let _bytes: [u8; 8] = [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]];
  return i64::from_be_bytes(_bytes);
}

pub fn write_long(data: &mut Vec<u8>, value: &i64) {
  let bytes = value.to_be_bytes();
  data.reserve(8);
  write_ubyte(data, &bytes[0]);
  write_ubyte(data, &bytes[1]);
  write_ubyte(data, &bytes[2]);
  write_ubyte(data, &bytes[3]);
  write_ubyte(data, &bytes[4]);
  write_ubyte(data, &bytes[5]);
  write_ubyte(data, &bytes[6]);
  write_ubyte(data, &bytes[7]);
}

pub fn read_ulong(data: &mut Vec<u8>) -> u64 {
  let bytes: Vec<u8> = data.drain(..8).collect();
  let _bytes: [u8; 8] = [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]];
  return u64::from_be_bytes(_bytes);
}

pub fn write_ulong(data: &mut Vec<u8>, value: &u64) {
  let bytes = value.to_be_bytes();
  data.reserve(8);
  write_ubyte(data, &bytes[0]);
  write_ubyte(data, &bytes[1]);
  write_ubyte(data, &bytes[2]);
  write_ubyte(data, &bytes[3]);
  write_ubyte(data, &bytes[4]);
  write_ubyte(data, &bytes[5]);
  write_ubyte(data, &bytes[6]);
  write_ubyte(data, &bytes[7]);
}

pub fn read_float(data: &mut Vec<u8>) -> f32 {
  let value = read_uint(data);
  return f32::from_bits(value);
}

pub fn write_float(data: &mut Vec<u8>, value: &f32) {
  write_uint(data, &value.to_bits());
}

pub fn read_double(data: &mut Vec<u8>) -> f64 {
  let value = read_ulong(data);
  return f64::from_bits(value);
}

pub fn write_double(data: &mut Vec<u8>, value: &f64) {
  write_ulong(data, &value.to_bits());
}

// Custom minecraft types

pub fn read_varint(data: &mut Vec<u8>) -> Option<i32> {
  let mut num_read = 0;
  let mut result = 0;
  let mut read;
  while {
    read = read_ubyte(data);
    let value: i32 = (read as i32) & 0b01111111;
    result |= value << (7 * num_read);
    num_read = num_read + 1;
    if num_read < 6 {}
    else {
      return None;
    }
    (read & 0b10000000) != 0
  } {}

  return Some(result);
}

pub fn write_varint(data: &mut Vec<u8>, value: &i32) {
  let mut temp = *value as u32;
  while {
    let mut byte: u8 = (temp as u8) & 0b01111111;
    temp = temp >> 7;

    if temp != 0 {
      byte |= 0b10000000;
    }
    write_ubyte(data, &byte);
    temp != 0
  } {}
}

pub fn read_varlong(data: &mut Vec<u8>) -> Option<i64> {
  let mut num_read = 0;
  let mut result = 0;
  let mut read;
  while {
    read = read_ubyte(data);
    let value: i64 = (read as i64) & 0b01111111;
    result |= value << (7 * num_read);
    num_read = num_read + 1;
    if num_read < 11 {}
    else {
      return None;
    }
    (read & 0b10000000) != 0
  } {}

  return Some(result);
}

pub fn write_varlong(data: &mut Vec<u8>, value: &i32) {
  let mut temp = *value as u32;
  while {
    let mut byte: u8 = (temp as u8) & 0b01111111;
    temp = temp >> 7;

    if temp != 0 {
      byte |= 0b10000000;
    }
    write_ubyte(data, &byte);
    temp != 0
  } {}
}

pub fn read_string(data: &mut Vec<u8>) -> Option<String> {
  match read_varint(data) {
    Some(length) => return String::from_utf8(data.drain(..(length as usize + 1)).collect()).ok(),
    None => return None
  }
}

pub fn write_string(data: &mut Vec<u8>, value: &str) {
  let len = value.len() as i32;
  write_varint(data, &len);
  data.reserve(value.len());
  data.extend_from_slice(value.as_bytes());
}

pub fn read_chat(data: &mut Vec<u8>) -> Option<Chat> {
  match read_string(data,) {
    Some(value) => return Some(json::from(value)),
    None => return None
  }
}

pub fn write_chat(data: &mut Vec<u8>, chat: &Chat) {
  let value = chat.as_str().unwrap();
  write_string(data, value);
}

pub fn read_uuid(data: &mut Vec<u8>) -> u128 {
  let msb = read_ulong(data) as u128;
  let lsb = read_ulong(data) as u128;
  return (msb << 64) | lsb;
}

pub fn write_uuid(data: &mut Vec<u8>, value: &u128) {
  let msb = (value >> 64) as u64;
  let lsb = (value & 0xFFFFFFFFFFFFFFFF) as u64;
  data.reserve(16);
  write_ulong(data, &msb);
  write_ulong(data, &lsb);
}

pub fn read_position(data: &mut Vec<u8>) -> Position {
  let long = read_ulong(data);

  let mut x: i32 = (long >> 38) as i32;
  let mut y: i32 = ((long >> 26) & 0xFFF) as i32;
  let mut z: i32 = ((long << 38) >> 38) as i32;
  if x >= 2^25 { x -= 2^26 }
  if y >= 2^11 { y -= 2^12 }
  if z >= 2^25 { z -= 2^26 }
  return Position {
    x: x,
    y: y,
    z: z
  };
}

pub fn write_position(data: &mut Vec<u8>, pos: &Position) {
  let long = (((pos.x as u64) & 0x3FFFFFF) << 38) | (((pos.y as u64) & 0xFFF) << 26) | ((pos.z as u64) & 0x3FFFFFF);
  write_ulong(data, &long);
}

pub fn read_nbt(data: &mut Vec<u8>) -> NBT {
  let result = NBT::from_raw(data);
  return result;
}

pub fn write_nbt(data: &mut Vec<u8>, value: &NBT) {
  data.extend(value.to_raw());
}

pub fn read_slot(data: &mut Vec<u8>) -> Slot {
  let mut slot = Slot { item_count: 0, item_id: None, nbt: None};
  let exists = read_ubyte(data);

  if exists == 1 {
    slot.item_id = read_varint(data);
    slot.item_count = read_byte(data);
    slot.nbt = Some(read_nbt(data));
  }
  return slot;
}

pub trait Packet {
  fn to_raw(&self) -> Vec<u8>;
  fn from_raw(data: &mut Vec<u8>) -> Self;
}
