use binary_utils::binary::{Reader, Writer};
use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::address::InternetAddress;

pub struct NewIncomingConn {
    pub server_address: InternetAddress,
    pub system_addresses: [InternetAddress; 20],
    pub ping_time: u64,
    pub pong_time: u64,
}

impl NewIncomingConn {
    pub fn new(
        server_address: InternetAddress,
        system_addresses: [InternetAddress; 20],
        ping_time: u64,
        pong_time: u64,
    ) -> NewIncomingConn {
        NewIncomingConn { server_address, system_addresses, ping_time, pong_time }
    }

    pub fn encode(&self, stream: &mut Writer) {
        stream.clear();
        stream.put_u8(PacketType::get_u8(PacketType::NewIncomingConn));
        self.server_address.put_address(stream);
        for system_address in &self.system_addresses {
            system_address.put_address(stream);
        }
        stream.put_u64_be(self.ping_time);
        stream.put_u64_be(self.pong_time);
    }

    pub fn decode(stream: &mut Reader) -> NewIncomingConn {
        let (server_address, offset) = InternetAddress::get_address(stream.remaining()).unwrap();
        stream.set_offset(stream.offset() + offset);
        let mut system_addresses: [InternetAddress; 20] = core::array::from_fn(|_| InternetAddress::new(4, "127.0.0.1".to_string(), 0));
        for i in 0..20 {
            let (system_address, offset) = InternetAddress::get_address(stream.remaining()).unwrap();
            stream.set_offset(stream.offset() + offset);
            system_addresses[i] = system_address;
        }
        let ping_time = stream.get_u64_be();
        let pong_time = stream.get_u64_be();

        NewIncomingConn { server_address, system_addresses, ping_time, pong_time }
    }
}
