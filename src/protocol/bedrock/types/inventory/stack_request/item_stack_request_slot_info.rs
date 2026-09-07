use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::inventory::full_container_name::FullContainerName;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct ItemStackRequestSlotInfo {
    pub container_name: FullContainerName,
    pub slot_id: u8,
    pub stack_id: i32,
}

impl ItemStackRequestSlotInfo {
    pub fn new(container_name: FullContainerName, slot_id: u8, stack_id: i32) -> ItemStackRequestSlotInfo {
        ItemStackRequestSlotInfo { container_name, slot_id, stack_id }
    }

    pub fn read(stream: &mut Reader) -> ItemStackRequestSlotInfo {
        let container_name = FullContainerName::read(stream);
        let slot_id = stream.get_u8();
        let stack_id = stream.get_i32_le();

        ItemStackRequestSlotInfo { container_name, slot_id, stack_id }
    }

    pub fn write(&self, stream: &mut Writer) {
        self.container_name.write(stream);
        stream.put_u8(self.slot_id);
        stream.put_i32_le(self.stack_id);
    }
}
