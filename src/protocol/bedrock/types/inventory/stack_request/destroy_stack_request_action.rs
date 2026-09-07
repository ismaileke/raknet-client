use binary_utils::binary::{Reader, Writer};
use crate::protocol::bedrock::types::inventory::stack_request::item_stack_request_slot_info::ItemStackRequestSlotInfo;

#[derive(serde::Serialize, Debug)]
pub struct DestroyStackRequestAction {
    pub count: u8,
    pub source: ItemStackRequestSlotInfo
}

impl DestroyStackRequestAction {
    pub fn new(count: u8, source: ItemStackRequestSlotInfo) -> DestroyStackRequestAction {
        DestroyStackRequestAction { count, source }
    }

    pub fn read(stream: &mut Reader) -> DestroyStackRequestAction {
        DestroyStackRequestAction {
            count: stream.get_u8(),
            source: ItemStackRequestSlotInfo::read(stream),
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u8(self.count);
        self.source.write(stream);
    }
}
