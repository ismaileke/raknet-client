use crate::protocol::bedrock::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::command::command_output_message::CommandOutputMessage;
use crate::protocol::bedrock::types::command::command_origin_data::CommandOriginData;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct CommandOutput {
    pub origin_data: CommandOriginData,
    pub output_type: String,
    pub success_count: u32,
    pub messages: Vec<CommandOutputMessage>,
    pub data: Option<String>
}

impl Packet for CommandOutput {
    fn id(&self) -> u16 {
        BedrockPacketType::IDCommandOutput.get_u8()
    }

    fn encode(&mut self, stream: &mut Writer) {
        PacketSerializer::put_command_origin_data(stream, &self.origin_data);
        PacketSerializer::put_string(stream, &self.output_type);
        stream.put_u32_le(self.success_count);
        stream.put_var_u32(self.messages.len() as u32);
        for message in &self.messages {
            Self::put_command_message(message, stream);
        }
        PacketSerializer::write_optional(stream, &self.data, |s, v| PacketSerializer::put_string(s, v));
    }

    fn decode(stream: &mut Reader) -> CommandOutput {
        let origin_data = PacketSerializer::get_command_origin_data(stream);
        let output_type = PacketSerializer::get_string(stream);
        let success_count = stream.get_u32_le();
        let size = stream.get_var_u32();
        let mut messages = Vec::new();
        for _ in 0..size {
            messages.push(Self::get_command_message(stream));
        }
        let data = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_string(s));

        CommandOutput { origin_data, output_type, success_count, messages, data }
    }
}

impl CommandOutput {
    pub const TYPE_LAST: &'static str = "lastoutput";
    pub const TYPE_SILENT: &'static str = "silent";
    pub const TYPE_ALL: &'static str = "alloutput";
    pub const TYPE_DATA_SET: &'static str = "dataset";

    fn get_command_message(stream: &mut Reader) -> CommandOutputMessage {
        let message_id = PacketSerializer::get_string(stream);
        let is_internal = stream.get_bool();
        let size = stream.get_var_u32();
        let mut parameters = Vec::new();
        for _ in 0..size {
            parameters.push(PacketSerializer::get_string(stream));
        }
        CommandOutputMessage { message_id, is_internal, parameters }
    }
    fn put_command_message(message: &CommandOutputMessage, stream: &mut Writer) {
        PacketSerializer::put_string(stream, &message.message_id);
        stream.put_bool(message.is_internal);
        stream.put_var_u32(message.parameters.len() as u32);
        for parameter in &message.parameters {
            PacketSerializer::put_string(stream, parameter);
        }
    }
}