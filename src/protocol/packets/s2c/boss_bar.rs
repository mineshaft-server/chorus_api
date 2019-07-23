use crate::protocol::util::Packet;
use crate::protocol::types::chat::Chat;
use crate::protocol::util::*;

extern crate log;

#[derive(Debug)]
pub enum BossBarAction {
  Add(Chat, f32, i32, i32, u8),
  Remove,
  UpdateHealth(f32),
  UpdateTitle(Chat),
  UpdateStyle(i32, i32),
  UpdateFlags(u8)
}

#[derive(Debug)]
pub struct BossBar {
  pub uuid: u128,
  pub action: BossBarAction,
}

impl Default for BossBar {
  fn default() -> Self {
    return BossBar {
      uuid: 0,
      action: BossBarAction::Remove
    }
  }
}

impl Packet for BossBar {
  fn to_raw(&self, packet_id: i32) -> Vec<u8>{
        let mut raw = Vec::new();
        write_varint(&mut raw, &packet_id);
        write_uuid(&mut raw, &self.uuid);
        match &self.action {
          BossBarAction::Add(title, health, color, division, flags) => {
            write_varint(&mut raw, &0);
            write_chat(&mut raw, title);
            write_float(&mut raw, health);
            write_varint(&mut raw, color);
            write_varint(&mut raw, division);
            write_ubyte(&mut raw, flags);
          },
          BossBarAction::Remove => write_varint(&mut raw, &1),
          BossBarAction::UpdateHealth(health) => {
            write_varint(&mut raw, &2);
            write_float(&mut raw, health);
          },
          BossBarAction::UpdateTitle(title) => {
            write_varint(&mut raw, &3);
            write_chat(&mut raw, title);
          },
          BossBarAction::UpdateStyle(color, dividers) => {
            write_varint(&mut raw, &4);
            write_varint(&mut raw, color);
            write_varint(&mut raw, dividers);
          },
          BossBarAction::UpdateFlags(flags) => {
            write_varint(&mut raw, &5);
            write_ubyte(&mut raw, flags);
          }
        }
        return raw;
      }

      fn from_raw(raw: &mut Vec<u8>) -> Option<Self> {
        let mut packet = Self::default();
        if raw.len() > 8 {
          packet.uuid = read_uuid(raw);
          match read_varint(raw, -1) {
            0 => {
              match read_chat(raw) {
                Some(title) => {
                  let health: f32;
                  let color: i32;
                  let division: i32;
                  let flags: u8;

                  if raw.len() > 3 {
                    health = read_float(raw);
                  } else {
                    log::error!(target: "packet read", "BossBar: Unable to read health float from buffer");
                    return None;
                  }

                  color = read_varint(raw, -1);
                  if color != -1 {}
                  else {
                    log::error!(target: "packet read", "BossBar: Unable to read color varint from buffer");
                    return None;
                  }

                  division = read_varint(raw, -1);
                  if division != -1 {}
                  else {
                    log::error!(target: "packet read", "BossBar: Unable to read division varint from buffer");
                    return None;
                  }

                  if raw.len() > 0 {
                    flags = read_ubyte(raw);
                  } else {
                    log::error!(target: "packet read", "BossBar: Not enough bytes in buffer to read flags ubyte");
                    return None;
                  }
                  packet.action = BossBarAction::Add(title, health, color, division, flags);
                },
                None => {
                  log::error!(target: "packet read", "BossBar: Unable to read title json from buffer");
                  return None;
                }
              }
            },
            1 => packet.action = BossBarAction::Remove,
            2 => {
                if raw.len() > 3 {
                  packet.action = BossBarAction::UpdateHealth(read_float(raw));
                } else {
                  log::error!(target: "packet read", "BossBar: Unable to read health float from buffer");
                  return None;
                }
            },
            3 => {
              if let Some(title) = read_chat(raw) {
                packet.action = BossBarAction::UpdateTitle(title);
              } else {
                log::error!(target: "packet read", "BossBar: Unable to read title json from buffer");
                return None;
              }
            },
            4 => {
              let color = read_varint(raw, -1);
              if color != -1 {}
              else {
                log::error!(target: "packet read", "BossBar: Unable to read color varint from buffer");
                return None;
              }

              let division = read_varint(raw, -1);
              if division != -1 {}
              else {
                log::error!(target: "packet read", "BossBar: Unable to read division varint from buffer");
                return None;
              }

              packet.action = BossBarAction::UpdateStyle(color, division);
            },
            5 => {
              if raw.len() > 0 {
                packet.action = BossBarAction::UpdateFlags(read_ubyte(raw));
              } else {
                log::error!(target: "packet read", "BossBar: Not enough bytes in buffer to read UpdateFlags action");
                return None;
              }
            }
            _unknown => {
              log::error!(target: "packet read", "BossBar: Unknown action received");
              return None
            }
          }
        } else {
          log::error!(target: "packet read", "BossBar: Not enough bytes in buffer for minimum packet size");
          return None;
        }
        return Some(packet);
      }
}