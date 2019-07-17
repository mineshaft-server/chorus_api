use crate::protocol::util::Packet;
use crate::protocol::packets::c2s::advancement_tab::AdvancementTab;

#[test]
pub fn test_packet() {
  let packet = AdvancementTab::default();
  let raw = packet.to_raw(12);
  println!("{:?} {:?}", packet, raw);
  assert!(false);
}