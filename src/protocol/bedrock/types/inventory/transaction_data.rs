use crate::protocol::bedrock::inventory_transaction::InventoryTransaction;
use crate::protocol::bedrock::types::inventory::mismatch_transaction_data::MismatchTransactionData;
use crate::protocol::bedrock::types::inventory::network_inventory_action::NetworkInventoryAction;
use crate::protocol::bedrock::types::inventory::normal_transaction_data::NormalTransactionData;
use crate::protocol::bedrock::types::inventory::release_item_transaction_data::ReleaseItemTransactionData;
use crate::protocol::bedrock::types::inventory::use_item_on_entity_transaction_data::UseItemOnEntityTransactionData;
use crate::protocol::bedrock::types::inventory::use_item_transaction_data::UseItemTransactionData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub enum TransactionData {
    Normal(NormalTransactionData),
    Mismatch(MismatchTransactionData),
    UseItem(UseItemTransactionData),
    ReleaseItem(ReleaseItemTransactionData),
    UseItemOnEntity(UseItemOnEntityTransactionData),
}

impl TransactionData {
    pub fn get_type_id(&self) -> u32 {
        match self {
            TransactionData::Normal(_) => InventoryTransaction::TYPE_NORMAL,
            TransactionData::Mismatch(_) => InventoryTransaction::TYPE_MISMATCH,
            TransactionData::UseItem(_) => InventoryTransaction::TYPE_USE_ITEM,
            TransactionData::ReleaseItem(_) => InventoryTransaction::TYPE_RELEASE_ITEM,
            TransactionData::UseItemOnEntity(_) => InventoryTransaction::TYPE_USE_ITEM_ON_ENTITY,
        }
    }

    pub fn get_actions(&self) -> &Vec<NetworkInventoryAction> {
        match self {
            TransactionData::Normal(r) => r.get_actions(),
            TransactionData::Mismatch(r) => r.get_actions(),
            TransactionData::UseItem(r) => r.get_actions(),
            TransactionData::ReleaseItem(r) => r.get_actions(),
            TransactionData::UseItemOnEntity(r) => r.get_actions(),
        }
    }

    pub fn get_actions_mut(&mut self) -> &mut Vec<NetworkInventoryAction> {
        match self {
            TransactionData::Normal(r) => r.get_actions_mut(),
            TransactionData::Mismatch(r) => r.get_actions_mut(),
            TransactionData::UseItem(r) => r.get_actions_mut(),
            TransactionData::ReleaseItem(r) => r.get_actions_mut(),
            TransactionData::UseItemOnEntity(r) => r.get_actions_mut(),
        }
    }

    pub fn encode_data(&self, stream: &mut Writer) {
        match self {
            TransactionData::Normal(r) => r.encode_data(stream),
            TransactionData::Mismatch(r) => r.encode_data(stream),
            TransactionData::UseItem(r) => r.encode_data(stream),
            TransactionData::ReleaseItem(r) => r.encode_data(stream),
            TransactionData::UseItemOnEntity(r) => r.encode_data(stream),
        }
    }

    pub fn decode_data(&mut self, stream: &mut Reader) {
        match self {
            TransactionData::Normal(r) => r.decode_data(stream),
            TransactionData::Mismatch(r) => r.decode_data(stream),
            TransactionData::UseItem(r) => r.decode_data(stream),
            TransactionData::ReleaseItem(r) => r.decode_data(stream),
            TransactionData::UseItemOnEntity(r) => r.decode_data(stream),
        }
    }

    pub fn decode(&mut self, stream: &mut Reader) {
        let has_value = stream.get_bool();
        if !has_value { return }
        let action_count = stream.get_var_u32();
        for _ in 0..action_count {
            let action = NetworkInventoryAction::read(stream);
            self.get_actions_mut().push(action);
        }
        self.decode_data(stream)
    }


    pub fn encode(&self, stream: &mut Writer) {
        let has_value = self.get_actions().len() > 0;
        stream.put_bool(has_value);
        if !has_value { return }
        stream.put_var_u32(self.get_actions().len() as u32);
        for action in self.get_actions() {
            action.write(stream);
        }
        self.encode_data(stream)
    }
}
