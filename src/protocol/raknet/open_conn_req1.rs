use binary_utils::binary::{Reader, Writer};
use crate::protocol::raknet::packet_ids::PacketType;

pub struct OpenConnReq1 {
    pub magic: [u8; 16],
    pub protocol: u8,
    pub mtu_size: u16,
}

impl OpenConnReq1 {
    pub fn new(magic: [u8; 16], protocol: u8, mtu_size: u16) -> OpenConnReq1 {
        OpenConnReq1 { magic, protocol, mtu_size }
    }

    pub fn encode(&self, stream: &mut Writer) {
        stream.clear();
        stream.put_u8(PacketType::get_u8(PacketType::OpenConnReq1));
        stream.put(&self.magic);
        stream.put_u8(self.protocol);

        let target = (self.mtu_size as usize).saturating_sub(28);
        if target > stream.len() {
            stream.resize(target, 0);
        }
    }

    pub fn decode(stream: &mut Reader) -> OpenConnReq1 {
        let magic = stream.get(16).try_into().unwrap();
        let protocol = stream.get_u8();
        let mtu_size = stream.remaining_byte_count() as u16;

        OpenConnReq1 { magic, protocol, mtu_size }
    }
}
