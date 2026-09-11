use binary_utils::binary::{Reader, Writer};
use crate::protocol::raknet::packet_ids::PacketType;

pub struct IncompatibleProtocol {
    pub server_protocol: u8,
    pub magic: [u8; 16],
    pub server_guid: u64
}

impl IncompatibleProtocol {
    pub fn new(server_protocol: u8, magic: [u8; 16], server_guid: u64) -> IncompatibleProtocol {
        IncompatibleProtocol { server_protocol, magic, server_guid }
    }

    pub fn encode(&self, stream: &mut Writer) {
        stream.clear();
        stream.put_u8(PacketType::get_u8(PacketType::IncompatibleProtocol));
        stream.put_u8(self.server_protocol);
        stream.put(&self.magic);
        stream.put_u64_be(self.server_guid);
    }

    pub fn decode(stream: &mut Reader) -> IncompatibleProtocol {
        let server_protocol = stream.get_u8();
        let magic = stream.get(16).try_into().unwrap();
        let server_guid = stream.get_u64_be();

        IncompatibleProtocol { server_protocol, magic, server_guid }
    }
}
