use std::collections::HashMap;
use crate::protocol::util;


pub type ByteArrayType = Vec<i8>;
pub type ListType = Vec<Tag>;
pub type CompoundType = HashMap<String, Tag>;
pub type IntArrayType = Vec<i32>;
pub type LongArrayType = Vec<i64>;

#[derive(PartialEq, Debug)]
pub enum Tag {
  TagEnd,
  TagByte(i8),
  TagShort(i16),
  TagInt(i32),
  TagLong(i64),
  TagFloat(f32),
  TagDouble(f64),
  TagByteArray(ByteArrayType),
  TagString(String),
  TagList(ListType),
  TagCompound(CompoundType),
  TagIntArray(IntArrayType),
  TagLongArray(LongArrayType),
  TagInvalid(String)
}

impl Tag {
  pub fn from_raw(raw: &mut Vec<u8>) -> Tag {
    let tag_type = raw.remove(0);
    return Tag::read_tag(raw, tag_type)
  }

  pub fn to_raw(&self) -> Vec<u8> {
    return match self {
      Tag::TagEnd =>  {
        let mut data: Vec<u8> = Vec::with_capacity(1);
        util::write_byte(&mut data, &(0 as i8));
        data
      },
      Tag::TagByte(value) => {
        let mut data: Vec<u8> = Vec::with_capacity(2);
        util::write_byte(&mut data, &(1 as i8));
        util::write_byte(&mut data, value);
        data
      },
      Tag::TagShort(value) => {
        let mut data: Vec<u8> = Vec::with_capacity(3);
        util::write_byte(&mut data, &(2 as i8));
        util::write_short(&mut data, value);
        data
      },
      Tag::TagInt(value) => {
        let mut data: Vec<u8> = Vec::with_capacity(5);
        util::write_byte(&mut data, &(3 as i8));
        util::write_int(&mut data, value);
        data
      },
      Tag::TagLong(value) => {
        let mut data: Vec<u8> = Vec::with_capacity(9);
        util::write_byte(&mut data, &(4 as i8));
        util::write_long(&mut data, value);
        data
      },
      Tag::TagFloat(value) => {
        let mut data: Vec<u8> = Vec::with_capacity(5);
        util::write_byte(&mut data, &(5 as i8));
        util::write_float(&mut data, value);
        data
      },
      Tag::TagDouble(value) => {
        let mut data: Vec<u8> = Vec::with_capacity(9);
        util::write_byte(&mut data, &(6 as i8));
        util::write_double(&mut data, value);
        data
      }
      Tag::TagByteArray(value) => {
        let mut data: Vec<u8> = Vec::with_capacity(value.len() + 1);
        util::write_byte(&mut data, &(7 as i8));
        for i in value {
          util::write_byte(&mut data, i);
        }
        data
      }
      Tag::TagString(value) => {
        let mut data: Vec<u8> = Vec::with_capacity(value.len() + 3);
        util::write_byte(&mut data, &(8 as i8));
        util::write_string(&mut data, value);
        data
      },
      Tag::TagList(value) => {
        let mut data: Vec<u8> = Vec::with_capacity(5);
        util::write_byte(&mut data, &(9 as i8));
        util::write_int(&mut data, &(value.len() as i32));
        for tag in value {
          data.extend(tag.to_raw());
        }
        data
      },
      Tag::TagCompound(value) => {
        let mut data: Vec<u8> = Vec::with_capacity(2);
        util::write_byte(&mut data, &(10 as i8));
        for (k, v) in value {
          util::write_byte(&mut data, &(8 as i8));
          util::write_string(&mut data, k);
          data.extend(v.to_raw());
        }
        util::write_byte(&mut data, &(0 as i8));
        data
      },
      Tag::TagIntArray(value) => {
        let mut data: Vec<u8> = Vec::with_capacity(5 + (value.len() * 4));
        for int in value {
          util::write_int(&mut data, int);
        }
        data
      },
      Tag::TagLongArray(value) => {
        let mut data: Vec<u8> = Vec::with_capacity(5 + (value.len() * 8));
        for long in value {
          util::write_long(&mut data, long);
        }
        data
      },
      Tag::TagInvalid(_) => Vec::new()
    }
  }

  fn read_tag(raw: &mut Vec<u8>, tag_type: u8) -> Tag {
    return match tag_type {
      0 => Tag::TagEnd,
      1 => Tag::TagByte(util::read_byte(raw)),
      2 => Tag::TagShort(util::read_short(raw)),
      3 => Tag::TagInt(util::read_int(raw)),
      4 => Tag::TagLong(util::read_long(raw)),
      5 => Tag::TagFloat(util::read_float(raw)),
      6 => Tag::TagDouble(util::read_double(raw)),
      7 => {
        let length = util::read_int(raw);
        if length > 0 {
          let array: Vec<i8> = raw.drain(..(length as usize)).map(|x| x as i8).collect();
          Tag::TagByteArray(array)
        } else {
          let array: Vec<i8> = Vec::new();
          Tag::TagByteArray(array)
        }
      }
      8 => {
        let length = util::read_short(raw);
        if length > 0 {
          Tag::TagString(String::from_utf8(raw.drain(..(length as usize + 1)).collect()).unwrap())
        } else {
          Tag::TagString(String::from(""))
        }
      },
      9 => {
        let tag_type = raw.remove(0);
        let count = util::read_int(raw) as usize;
        Tag::read_list_raw(raw, tag_type, count)
      },
      10 => Tag::read_compound_raw(raw),
      11 => Tag::read_int_array(raw),
      12 => Tag::read_long_array(raw),
      _ => Tag::TagInvalid(format!("Tried reading invalid tag with type {}", tag_type))
    }
  }

  fn read_list_raw(raw: &mut Vec<u8>, tag_type: u8, count: usize) -> Tag {
    let mut tags: Vec<Tag> = Vec::with_capacity(count);
    for _ in 0..count {
     tags.push(Tag::read_tag(raw, tag_type));
    }

    return Tag::TagList(tags);
  }

  fn read_compound_raw(raw: &mut Vec<u8>) -> Tag {
    let mut compound: HashMap<String, Tag> = HashMap::new();
    let mut key: Tag = Tag::from_raw(raw);
    let mut do_read = true;
    while {
      match key {
        Tag::TagString(actual_key) => {
          compound.insert(actual_key, Tag::from_raw(raw));
          key = Tag::from_raw(raw);
        },
        Tag::TagEnd => do_read = false,
        Tag::TagInvalid(_) => return key,
        _ => return Tag::TagInvalid(String::from("Got invalid tag for compound key."))
      }
      do_read
    } {}

    return Tag::TagCompound(compound);
  }

  fn read_int_array(raw: &mut Vec<u8>) -> Tag {
    let count = util::read_int(raw) as usize;
    let expected_bits = count * 4;
    let mut array: Vec<i32> = Vec::new();
    if count > 0 {
      if (count * 4) <= raw.len() {
        array.reserve(count);
        for _ in 0..count {
          array.push(util::read_int(raw));
        }
      } else {
        return Tag::TagInvalid(format!("Tried reading int array with expected bit count of {}, got {} instead", expected_bits, raw.len()));
      }
    }

    return Tag::TagIntArray(array);
  }

  fn read_long_array(raw: &mut Vec<u8>) -> Tag {
    let count = util::read_int(raw) as usize;
    let expected_bits = count * 4;
    let mut array: Vec<i64> = Vec::new();
    if count > 0 {
      if (count * 4) <= raw.len() {
        array.reserve(count);
        for _ in 0..count {
          array.push(util::read_long(raw));
        }
      } else {
        return Tag::TagInvalid(format!("Tried reading long array with expected bit count of {}, got {} instead", expected_bits, raw.len()));
      }
    }

    return Tag::TagLongArray(array);
  }
}

impl Default for Tag {
  fn default() -> Self { Tag::TagEnd }
}