use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::address::InternetAddress;
use crate::utils::color_format::COLOR_WHITE;
use crate::utils::color_format;
use binary_utils::binary::{Reader, Writer};

pub struct OpenConnReply2 {
    pub magic: [u8; 16],
    pub server_guid: u64,
    pub client_address: InternetAddress,
    pub mtu: u16,
    pub encryption_enabled: bool,
}

impl OpenConnReply2 {
    pub fn encode(&self, stream: &mut Writer) {
        stream.clear();
        stream.put_u8(PacketType::get_u8(PacketType::OpenConnReply2));
        stream.put(&self.magic);
        stream.put_u64_be(self.server_guid);
        self.client_address.put_address(stream);
        stream.put_u16_be(self.mtu);
        stream.put_bool(self.encryption_enabled);
    }

    pub fn decode(stream: &mut Reader) -> OpenConnReply2 {
        let magic = stream.get(16).try_into().unwrap();
        let server_guid = stream.get_u64_be();
        let (client_address, offset) = InternetAddress::get_address(stream.remaining()).unwrap();
        stream.set_offset(stream.offset() + offset);
        let mtu = stream.get_u16_be();
        let encryption_enabled = stream.get_bool();

        OpenConnReply2 { magic, server_guid, client_address, mtu, encryption_enabled }
    }

    pub fn debug(&self) {
        println!("--- {}OpenConnReply2{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Magic: {:?}", self.magic);
        println!("Server GUID (Format DecToHex): {}", format!("{:x}", self.server_guid));
        println!("Client Address: {}:{}", self.client_address.address, self.client_address.port);
        println!("MTU: {}", self.mtu);
        println!("Encryption Enabled: {}", self.encryption_enabled);
    }
}
