use crate::protocol::util::*;
use crate::util::identifier::Identifier;

#[derive(Debug,Default)]
pub struct PluginMessage {
  pub channel: Identifier,
  pub data: Vec<u8>
}

impl Packet for PluginMessage {
  fn to_raw(&self, packet_id: i32) -> Vec<u8> {
    let id = self.channel.to_string();
    let mut raw = Vec::with_capacity(id.len() + self.data.len() + 3);
    write_varint(&mut raw, &packet_id);
    write_string(&mut raw, &id);
    raw.extend(&self.data);
    return raw;
  }

  fn from_raw(raw: &mut Vec<u8>) -> Option<Self> {
    let mut packet = Self::default();
    let tmp = crate::protocol::util::read_string(raw, String::from(""));

    // Verify Read Identifier
    if tmp != "" {
      packet.channel = Identifier::from(&tmp);
    } else {
      log::error!(target: "packet read", "Unable to read identifier from buffer");
      return None;
    }
    packet.data = raw[..].iter().cloned().collect();

    return Some(packet);
  }
}