use binary_utils::binary::{Reader, Writer};
use crate::utils::color_format;
use crate::utils::color_format::COLOR_WHITE;
use crate::protocol::raknet::packet_ids::PacketType;

pub struct OpenConnReply1 {
    pub magic: [u8; 16],
    pub server_guid: u64,
    pub server_security: bool,
    pub cookie: Option<u32>,
    pub mtu: u16
}

impl OpenConnReply1 {
    pub fn encode(&self, stream: &mut Writer) {
        stream.clear();
        stream.put_u8(PacketType::get_u8(PacketType::OpenConnReply1));
        stream.put(&self.magic);
        stream.put_u64_be(self.server_guid);
        stream.put_bool(self.server_security);
        if let Some(cookie) = self.cookie {
            stream.put_u32_be(cookie);
        }
        stream.put_u16_be(self.mtu);
    }
    
    pub fn decode(stream: &mut Reader) -> OpenConnReply1 {
        let magic = stream.get(16).try_into().unwrap();
        let server_guid = stream.get_u64_be();
        let server_security = stream.get_bool();
        let mut cookie = None;
        if server_security {
            cookie = Some(stream.get_u32_be());
        }
        let mtu = stream.get_u16_be();

        OpenConnReply1 { magic, server_guid, server_security, cookie, mtu }
    }

    pub fn debug(&self) {
        println!("--- {}OpenConnReply1{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Magic: {:?}", self.magic);
        let guid_format = format!("{:x}", self.server_guid);
        println!("Server GUID (Format DecToHex): {}", guid_format);
        println!("Server Security: {}", self.server_security);
        println!("Cookie: {:?}", self.cookie);
        println!("MTU: {}", self.mtu);
    }
}
