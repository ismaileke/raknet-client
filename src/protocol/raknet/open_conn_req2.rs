use binary_utils::binary::{Reader, Writer};
use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::address::InternetAddress;

pub struct OpenConnReq2 {
    pub magic: [u8; 16],
    pub server_address: InternetAddress,
    pub cookie: Option<u32>,
    pub client_supports_security: bool,
    pub mtu: u16,
    pub client_guid: u64,
}

impl OpenConnReq2 {
    const TAIL_FIELDS_SIZE_COMMON: u32 = 2 + 8; //mtu + client guid
    const TAIL_FIELDS_SIZE_IPV4: u32 = OpenConnReq2::TAIL_FIELDS_SIZE_COMMON + 7; // 1: type, 4: ip, 2: port
    const TAIL_FIELDS_SIZE_IPV6: u32 = OpenConnReq2::TAIL_FIELDS_SIZE_COMMON + 29; // 1: type, 2: family, 2: port, 4: flow info, 16: ip, 4: scope ID

    pub fn new(
        magic: [u8; 16],
        server_address: InternetAddress,
        cookie: Option<u32>,
        client_supports_security: bool,
        mtu: u16,
        client_guid: u64,
    ) -> OpenConnReq2 {
        OpenConnReq2 { magic, server_address, cookie, client_supports_security, mtu, client_guid }
    }

    pub fn encode(&self, stream: &mut Writer) {
        stream.clear();
        stream.put_u8(PacketType::get_u8(PacketType::OpenConnReq2));
        stream.put(&self.magic);
        if let Some(cookie) = self.cookie {
            stream.put_u32_be(cookie);
            stream.put_bool(self.client_supports_security);
        }
        self.server_address.put_address(stream);
        stream.put_u16_be(self.mtu);
        stream.put_u64_be(self.client_guid);
    }

    pub fn decode(stream: &mut Reader) -> OpenConnReq2 {
        let magic = stream.get(16).try_into().unwrap();

        let mut cookie: Option<u32> = None;
        let mut client_supports_security = false;

        let remaining_len = stream.remaining_byte_count() as u32;
        if remaining_len != OpenConnReq2::TAIL_FIELDS_SIZE_IPV4 && remaining_len != OpenConnReq2::TAIL_FIELDS_SIZE_IPV6 {
            cookie = Option::from(stream.get_u32_be());
            client_supports_security = stream.get_bool();
        }
        let (server_address, offset) = InternetAddress::get_address(stream.remaining()).expect("Invalid address");
        stream.set_offset(stream.offset() + offset);
        let mtu = stream.get_u16_be();
        let client_guid = stream.get_u64_be();

        OpenConnReq2 { magic, server_address, cookie, client_supports_security, mtu, client_guid }
    }
}
