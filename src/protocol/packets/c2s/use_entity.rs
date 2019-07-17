
use crate::protocol::util::*;

#[derive(Debug,Default)]
pub struct UseEntity {
  pub target: i32,
  pub type_: i32,
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub hand: i32
}

impl Packet for UseEntity {
  fn to_raw(&self, packet_id: i32) -> Vec<u8> {
    let mut raw = Vec::with_capacity(17);
    write_varint(&mut raw, &packet_id);
    write_varint(&mut raw, &self.target);
    write_varint(&mut raw, &self.type_);
    if self.type_ == 2 {
      write_float(&mut raw, &self.x);
      write_float(&mut raw, &self.y);
      write_float(&mut raw, &self.z);
      write_varint(&mut raw, &self.hand);
    } else if self.type_ == 0 {
      write_varint(&mut raw, &self.hand);
    }
    return raw;
  }

  fn from_raw(raw: &mut Vec<u8>) -> Option<Self> {
    let mut packet = Self::default();

    packet.target = read_varint(raw, -1);
    if packet.target != -1 {
    } else {
      return None;
    }

    packet.type_ = read_varint(raw, -1);
    if packet.type_ != -1 {
    } else {
      return None;
    }

    if packet.type_ == 0 {
      packet.hand = read_varint(raw, -1);
      if packet.hand != -1 {
      } else {
        return None;
      }
    } else if packet.type_ == 2 {
      if raw.len() > 12 {
        packet.x = read_float(raw);
        packet.y = read_float(raw);
        packet.z = read_float(raw);
        packet.hand = read_varint(raw, -1);
        if packet.hand != -1 {
        } else {
          return None;
        }
      } else {
          return None;
      }
    }

    return Some(packet);
  }
}