use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::address::InternetAddress;
use crate::utils::color_format::COLOR_WHITE;
use crate::utils::color_format;
use binary_utils::binary::{Reader, Writer};

pub struct ConnReqAccepted {
    pub client_address: InternetAddress,
    pub system_index: u16,
    pub system_addresses: [InternetAddress; 10],
    pub ping_time: u64,
    pub pong_time: u64,
}

impl ConnReqAccepted {
    pub fn encode(&self, stream: &mut Writer) {
        stream.clear();
        stream.put_u8(PacketType::get_u8(PacketType::ConnReqAccepted));
        self.client_address.put_address(stream);
        stream.put_u16_be(self.system_index);
        for system_address in &self.system_addresses {
            system_address.put_address(stream);
        }
        stream.put_u64_be(self.ping_time);
        stream.put_u64_be(self.pong_time);
    }
    
    pub fn decode(stream: &mut Reader) -> ConnReqAccepted {
        let (client_address, offset) = InternetAddress::get_address(stream.remaining()).unwrap();
        stream.set_offset(stream.offset() + offset);
        let system_index = stream.get_u16_be();
        let mut system_addresses: [InternetAddress; 10] = core::array::from_fn(|_| InternetAddress::new(4, "127.0.0.1".to_string(), 0));
        if stream.remaining_byte_count() > 16 {
            for index in 0..10 {
                let (system_address, offset) = InternetAddress::get_address(stream.remaining()).unwrap();
                stream.set_offset(stream.offset() + offset);
                system_addresses[index] = system_address;
            }
        }

        let ping_time = stream.get_u64_be();
        let pong_time = stream.get_u64_be();

        ConnReqAccepted { client_address, system_index, system_addresses, ping_time, pong_time }
    }

    pub fn debug(&self) {
        println!("--- {}ConnectionRequestAccepted{} ---", color_format::COLOR_GOLD, COLOR_WHITE);
        println!("Client Address: {}:{}", self.client_address.address, self.client_address.port);
        println!("System Index: {}", self.system_index);
        for index in 0..10 {
            println!("System Address {}: {}:{}", index + 1, self.system_addresses[index].address, self.system_addresses[index].port);
        }
        println!("Ping Time: {}", self.ping_time);
        println!("Pong Time: {}", self.ping_time);
    }
}
