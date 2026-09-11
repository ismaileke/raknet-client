use binary_utils::binary::{Reader, Writer};
use crate::protocol::raknet::packet_ids::PacketType;

pub struct ConnReq {
    pub client_guid: u64,
    pub request_time: u64,
    pub secure: bool,
}

impl ConnReq {
    pub fn new(client_guid: u64, request_time: u64, secure: bool) -> ConnReq {
        ConnReq { client_guid, request_time, secure }
    }

    pub fn encode(&self, stream: &mut Writer) {
        stream.clear();
        stream.put_u8(PacketType::get_u8(PacketType::ConnReq));
        stream.put_u64_be(self.client_guid);
        stream.put_u64_be(self.request_time);
        stream.put_bool(self.secure);
    }

    pub fn decode(stream: &mut Reader) -> ConnReq {
        let client_guid = stream.get_u64_be();
        let request_time = stream.get_u64_be();
        let secure = stream.get_bool();

        ConnReq { client_guid, request_time, secure }
    }
}
