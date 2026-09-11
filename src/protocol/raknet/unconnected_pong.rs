use binary_utils::binary::{Reader, Writer};
use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::color_format;
use crate::utils::color_format::COLOR_WHITE;
use crate::protocol::raknet::packet_ids;

pub struct UnconnectedPong {
    pub pong_time: u64,
    pub server_id: u64,
    pub server_name: String
}

impl UnconnectedPong {
    pub fn create(pong_time: u64, server_id: u64, server_name: String) -> UnconnectedPong {
        UnconnectedPong { pong_time, server_id, server_name }
    }

    pub fn encode(&self, stream: &mut Writer) {
        stream.clear();
        stream.put_u8(PacketType::get_u8(PacketType::UnconnectedPong));
        stream.put_u64_be(self.pong_time);
        stream.put_u64_be(self.server_id);
        stream.put(&packet_ids::MAGIC);
        stream.put_u16_be(self.server_name.len() as u16);
        stream.put(self.server_name.as_bytes());
    }

    pub fn decode(stream: &mut Reader) -> UnconnectedPong {
        let pong_time = stream.get_u64_be();
        let server_id = stream.get_u64_be();
        let _ = stream.get(16);
        let len = stream.get_u16_be();
        let server_name = String::from_utf8(Vec::from(stream.get(len as usize))).expect("Vec<u8> to String UTF8 conversion failed");

        UnconnectedPong { pong_time, server_id, server_name }
    }

    pub fn debug(&self) {
        println!("--- {}UnconnectedPong{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Pong Time: {:?}", self.pong_time);
        println!("Server ID: {:?}", self.server_id);
        println!("Server Name: {:?}", self.server_name);
    }
}
