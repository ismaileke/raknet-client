use binary_utils::binary::{Reader, Writer};
use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::color_format;
use crate::utils::color_format::COLOR_WHITE;

pub struct ConnectedPing {
    pub ping_time: u64
}

impl ConnectedPing {
    pub fn create(ping_time: u64) -> ConnectedPing {
        ConnectedPing { ping_time }
    }

    pub fn encode(&self, stream: &mut Writer) {
        stream.clear();
        stream.put_u8(PacketType::get_u8(PacketType::ConnectedPing));
        stream.put_u64_be(self.ping_time);
    }

    pub fn decode(stream: &mut Reader) -> ConnectedPing {
        let ping_time = stream.get_u64_be();
        ConnectedPing { ping_time }
    }

    pub fn debug(&self) {
        println!("--- {}ConnectedPing{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Ping Time: {:?}", self.ping_time);
    }
}
