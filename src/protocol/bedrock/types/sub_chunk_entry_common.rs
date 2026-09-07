use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use crate::protocol::bedrock::types::sub_chunk_height_map_info::SubChunkHeightMapInfo;
use crate::protocol::bedrock::types::sub_chunk_height_map_type::SubChunkHeightMapType;
use crate::protocol::bedrock::types::sub_chunk_position_offset::SubChunkPositionOffset;
use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct SubChunkEntryCommon {
    pub offset: SubChunkPositionOffset,
    pub request_result: u8,
    pub terrain_data: Option<Vec<u8>>,
    pub height_map: Option<SubChunkHeightMapInfo>,
    pub render_height_map: Option<SubChunkHeightMapInfo>,
}

impl SubChunkEntryCommon {
    pub fn new(
        offset: SubChunkPositionOffset,
        request_result: u8,
        terrain_data: Option<Vec<u8>>,
        height_map: Option<SubChunkHeightMapInfo>,
        render_height_map: Option<SubChunkHeightMapInfo>,
    ) -> SubChunkEntryCommon {
        SubChunkEntryCommon {
            offset,
            request_result,
            terrain_data,
            height_map,
            render_height_map,
        }
    }

    pub fn read(stream: &mut Reader) -> SubChunkEntryCommon {
        let offset = SubChunkPositionOffset::read(stream);
        let request_result = stream.get_u8();
        let terrain_data = PacketSerializer::read_optional(stream, |s| PacketSerializer::get_byte_string(s));
        let height_map_data_type = stream.get_u8();
        let height_map_data = PacketSerializer::read_optional(stream, |s| SubChunkHeightMapInfo::read(s));
        let height_map = match height_map_data_type {
            SubChunkHeightMapType::NO_DATA => None,
            SubChunkHeightMapType::DATA => height_map_data,
            SubChunkHeightMapType::ALL_TOO_HIGH => Some(SubChunkHeightMapInfo::all_too_high()),
            SubChunkHeightMapType::ALL_TOO_LOW => Some(SubChunkHeightMapInfo::all_too_low()),
            _ => panic!("Unknown heightmap data type {}", height_map_data_type),
        };

        let render_height_map_data_type = stream.get_u8();
        let render_height_map_data = PacketSerializer::read_optional(stream, |s| SubChunkHeightMapInfo::read(s));
        let render_height_map = match render_height_map_data_type {
            SubChunkHeightMapType::NO_DATA => None,
            SubChunkHeightMapType::DATA => render_height_map_data,
            SubChunkHeightMapType::ALL_TOO_HIGH => Some(SubChunkHeightMapInfo::all_too_high()),
            SubChunkHeightMapType::ALL_TOO_LOW => Some(SubChunkHeightMapInfo::all_too_low()),
            SubChunkHeightMapType::ALL_COPIED => height_map.clone(),
            _ => panic!("Unknown render heightmap data type {}", height_map_data_type),
        };

        SubChunkEntryCommon {
            offset,
            request_result,
            terrain_data,
            height_map,
            render_height_map,
        }
    }

    pub fn write(&self, stream: &mut Writer) {
        self.offset.write(stream);
        stream.put_u8(self.request_result);
        PacketSerializer::write_optional(stream, &self.terrain_data, |s, v| PacketSerializer::put_byte_string(s, v));
        if let Some(height_map) = &self.height_map {
            if height_map.is_all_too_low() {
                stream.put_u8(SubChunkHeightMapType::ALL_TOO_LOW);
                stream.put_bool(false);
            } else if height_map.is_all_too_high() {
                stream.put_u8(SubChunkHeightMapType::ALL_TOO_HIGH);
                stream.put_bool(false);
            } else {
                stream.put_u8(SubChunkHeightMapType::DATA);
                stream.put_bool(true);
                height_map.write(stream);
            }
        } else {
            stream.put_u8(SubChunkHeightMapType::NO_DATA);
            stream.put_bool(false);
        }

        if let Some(render_height_map) = &self.render_height_map {
            if render_height_map.is_all_too_low() {
                stream.put_u8(SubChunkHeightMapType::ALL_TOO_LOW);
                stream.put_bool(false);
            } else if render_height_map.is_all_too_high() {
                stream.put_u8(SubChunkHeightMapType::ALL_TOO_HIGH);
                stream.put_bool(false);
            } else {
                stream.put_u8(SubChunkHeightMapType::DATA);
                stream.put_bool(true);
                render_height_map.write(stream);
            }
        } else {
            stream.put_u8(SubChunkHeightMapType::ALL_COPIED);
            stream.put_bool(false);
        }
    }
}
