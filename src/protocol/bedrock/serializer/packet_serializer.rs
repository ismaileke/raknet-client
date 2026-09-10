use crate::protocol::bedrock::types::bool_game_rule::BoolGameRule;
use crate::protocol::bedrock::types::cacheable_nbt::CacheableNBT;
use crate::protocol::bedrock::types::command::command_origin_data::CommandOriginData;
use crate::protocol::bedrock::types::entity::entity_link::EntityLink;
use crate::protocol::bedrock::types::entity::entity_metadata_types::EntityMetadataTypes;
use crate::protocol::bedrock::types::entity::metadata_property::MetadataProperty;
use crate::protocol::bedrock::types::float_game_rule::FloatGameRule;
use crate::protocol::bedrock::types::game_rule::GameRule;
use crate::protocol::bedrock::types::game_rule_types::GameRuleTypes;
use crate::protocol::bedrock::types::int_game_rule::IntGameRule;
use crate::protocol::bedrock::types::inventory::item_stack::ItemStack;
use crate::protocol::bedrock::types::inventory::item_stack_wrapper::ItemStackWrapper;
use crate::protocol::bedrock::types::recipe::item_descriptor::ItemDescriptor;
use crate::protocol::bedrock::types::recipe::item_descriptor_type::ItemDescriptorType;
use crate::protocol::bedrock::types::recipe::molang_item_descriptor::MolangItemDescriptor;
use crate::protocol::bedrock::types::recipe::default_item_descriptor::DefaultItemDescriptor;
use crate::protocol::bedrock::types::recipe::recipe_ingredient::RecipeIngredient;
use crate::protocol::bedrock::types::recipe::tag_item_descriptor::TagItemDescriptor;
use crate::protocol::bedrock::types::skin::persona_piece_tint_color::PersonaPieceTintColor;
use crate::protocol::bedrock::types::skin::persona_skin_piece::PersonaSkinPiece;
use crate::protocol::bedrock::types::skin::skin_animation::SkinAnimation;
use crate::protocol::bedrock::types::skin::skin_data::SkinData;
use crate::protocol::bedrock::types::skin::skin_image::SkinImage;
use crate::protocol::bedrock::types::structure_editor_data::StructureEditorData;
use crate::protocol::bedrock::types::structure_settings::StructureSettings;
use binary_utils::binary::{Reader, Writer};
use mojang_nbt::nbt_serializer::NBTReader;
use mojang_nbt::tag::compound_tag::CompoundTag;
use mojang_nbt::tag::tag::Tag;
use mojang_nbt::tree_root::TreeRoot;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(serde::Serialize, Debug)]
pub struct PacketSerializer {}

impl PacketSerializer {
    /*pub fn get_string(stream: &mut Stream) -> String {
        let length = stream.get_var_u32();
        let bytes = stream.get(length);
        String::from_utf8(bytes).expect("Vec<u8> to String UTF8 conversion failed")
    }*/
    pub fn get_string(stream: &mut Reader) -> String {
        let length = stream.get_var_u32();
        let bytes = stream.get(length as usize);

        if let Ok(s) = String::from_utf8(Vec::from(bytes)) {
            return s;
        }

        String::from_utf8_lossy(&bytes).to_string()
    }

    pub fn put_string(stream: &mut Writer, data: &str) {
        stream.put_var_u32(data.len() as u32);
        stream.put(data.as_bytes());
    }

    pub fn get_byte_string(stream: &mut Reader) -> Vec<u8> {
        let length = stream.get_var_u32();
        stream.get(length as usize).to_vec()
    }

    pub fn put_byte_string(stream: &mut Writer, data: &[u8]) {
        stream.put_var_u32(data.len() as u32);
        stream.put(data);
    }

    pub fn get_uuid(stream: &mut Reader) -> String {
        let mut bytes = [0u8; 16];

        let p1 = stream.get(8);
        let p2 = stream.get(8);
        bytes[..8].copy_from_slice(p1);
        bytes[..8].reverse();
        bytes[8..].copy_from_slice(p2);
        bytes[8..].reverse();

        Uuid::from_bytes(bytes).to_string()
    }

    pub fn put_uuid(stream: &mut Writer, data: &str) {
        let uuid = Uuid::parse_str(data).expect("Invalid UUID format");
        let bytes = uuid.into_bytes();

        let mut p1 = [0u8; 8];
        p1.copy_from_slice(&bytes[..8]);
        p1.reverse();

        let mut p2 = [0u8; 8];
        p2.copy_from_slice(&bytes[8..]);
        p2.reverse();

        stream.put(&p1);
        stream.put(&p2);
    }

    pub fn get_actor_unique_id(stream: &mut Reader) -> i64 {
        stream.get_var_i64()
    }

    pub fn put_actor_unique_id(stream: &mut Writer, data: i64) {
        stream.put_var_i64(data);
    }

    pub fn get_actor_runtime_id(stream: &mut Reader) -> u64 {
        stream.get_var_u64()
    }

    pub fn put_actor_runtime_id(stream: &mut Writer, data: u64) {
        stream.put_var_u64(data);
    }

    pub fn get_vector3(stream: &mut Reader) -> Vec<f32> {
        let x = stream.get_f32_le();
        let y = stream.get_f32_le();
        let z = stream.get_f32_le();
        vec![x, y, z]
    }

    pub fn put_vector3(stream: &mut Writer, data: &Vec<f32>) {
        stream.put_f32_le(data[0]);
        stream.put_f32_le(data[1]);
        stream.put_f32_le(data[2]);
    }

    pub fn put_vector3_nullable(stream: &mut Writer, data: Option<Vec<f32>>) {
        if let Some(data) = &data {
            PacketSerializer::put_vector3(stream, data);
        } else {
            stream.put_f32_le(0.0);
            stream.put_f32_le(0.0);
            stream.put_f32_le(0.0);
        }
    }

    pub fn get_vector2(stream: &mut Reader) -> Vec<f32> {
        let x = stream.get_f32_le();
        let y = stream.get_f32_le();
        vec![x, y]
    }

    pub fn put_vector2(stream: &mut Writer, data: &Vec<f32>) {
        stream.put_f32_le(data[0]);
        stream.put_f32_le(data[1]);
    }

    pub fn get_block_pos(stream: &mut Reader) -> Vec<i32> {
        let x = stream.get_var_i32();
        let y = stream.get_var_i32();
        let z = stream.get_var_i32();
        vec![x, y, z]
    }

    pub fn put_block_pos(stream: &mut Writer, data: &Vec<i32>) {
        stream.put_var_i32(data[0]);
        stream.put_var_i32(data[1]);
        stream.put_var_i32(data[2]);
    }

    pub fn get_rotation_byte(stream: &mut Reader) -> f32 {
        (stream.get_u8() as f32) * (360f32 / 256f32)
    }

    pub fn put_rotation_byte(stream: &mut Writer, data: f32) {
        stream.put_u8((data / (360f32 / 256f32)) as u8);
    }

    pub fn get_entity_link(stream: &mut Reader) -> EntityLink {
        let from_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let to_actor_unique_id = PacketSerializer::get_actor_unique_id(stream);
        let action_type = stream.get_u8();
        let immediate = stream.get_bool();
        let caused_by_rider = stream.get_bool();
        let vehicle_angular_velocity = stream.get_f32_le();
        EntityLink::new(
            from_actor_unique_id,
            to_actor_unique_id,
            action_type,
            immediate,
            caused_by_rider,
            vehicle_angular_velocity,
        )
    }

    pub fn put_entity_link(stream: &mut Writer, data: &EntityLink) {
        PacketSerializer::put_actor_unique_id(stream, data.from_actor_unique_id);
        PacketSerializer::put_actor_unique_id(stream, data.to_actor_unique_id);
        stream.put_u8(data.action_type);
        stream.put_bool(data.immediate);
        stream.put_bool(data.caused_by_rider);
        stream.put_f32_le(data.vehicle_angular_velocity);
    }

    pub fn get_nbt_root<'a>(stream: &mut Reader<'a>) -> TreeRoot<'a> {
        let mut offset = stream.offset();
        let mut nbt_serializer = NBTReader::new_network();
        let nbt_root = nbt_serializer.read(stream.get_buffer(), &mut offset, 0);
        stream.set_offset(offset);
        nbt_root
    }

    pub fn get_nbt_compound_root(stream: &mut Reader) -> CompoundTag {
        let ct = PacketSerializer::get_nbt_root(stream).must_get_compound_tag().expect("get_nbt_compound_root() error");
        ct
    }

    pub fn get_entity_metadata(stream: &mut Reader) -> HashMap<u32, MetadataProperty> {
        let count = stream.get_var_u32() as usize;
        let mut data = HashMap::new();
        for _ in 0..count {
            let key = stream.get_var_u32();
            let metadata_type = stream.get_var_u32();
            let _ = stream.get_u8();
            data.insert(key, Self::read_metadata_property(stream, metadata_type));
        }

        data
    }

    fn read_metadata_property(stream: &mut Reader, metadata_type: u32) -> MetadataProperty {
        match metadata_type {
            EntityMetadataTypes::BYTE => MetadataProperty::Byte(stream.get_u8()),
            EntityMetadataTypes::SHORT => MetadataProperty::Short(stream.get_i16_le()),
            EntityMetadataTypes::INT => MetadataProperty::Int(stream.get_var_i32()),
            EntityMetadataTypes::FLOAT => MetadataProperty::Float(stream.get_f32_le()),
            EntityMetadataTypes::STRING => MetadataProperty::String(PacketSerializer::get_string(stream)),
            EntityMetadataTypes::COMPOUND_TAG => MetadataProperty::CompoundTag(CacheableNBT::new(Tag::Compound(PacketSerializer::get_nbt_compound_root(stream)))),
            EntityMetadataTypes::BLOCK_POS => MetadataProperty::BlockPos(PacketSerializer::get_block_pos(stream)),
            EntityMetadataTypes::LONG => MetadataProperty::Long(stream.get_var_i64()),
            EntityMetadataTypes::VECTOR3F => MetadataProperty::Vector3f(PacketSerializer::get_vector3(stream)),
            _ => panic!("Unknown metadata type id: {}", metadata_type),
        }
    }

    pub fn put_entity_metadata(stream: &mut Writer, data: &mut HashMap<u32, MetadataProperty>) {
        stream.put_var_u32(data.len() as u32);
        for (key, value) in data.iter_mut() {
            stream.put_var_u32(*key);
            stream.put_var_u32(value.id());
            stream.put_u8(value.id() as u8);
            value.write(stream);
        }
    }

    pub fn read_recipe_net_id(stream: &mut Reader) -> i32 {
        stream.get_var_i32()
    }

    pub fn write_recipe_net_id(stream: &mut Writer, id: i32) {
        stream.put_var_i32(id);
    }

    pub fn read_creative_item_net_id(stream: &mut Reader) -> u32 {
        stream.get_var_u32()
    }

    pub fn write_creative_item_net_id(stream: &mut Writer, id: u32) {
        stream.put_var_u32(id);
    }

    /**
     * This is a union of ItemStackRequestId, LegacyItemStackRequestId, and ServerItemStackId, used in server-bound
     * packets to allow the client to refer to server-known items, or items which may have been modified by a previous
     * as-yet unacknowledged request from the client.
     *
     * - Server item stack ID is positive
     * - InventoryTransaction "legacy" request ID is negative and even
     * - ItemStackRequest request ID is negative, and odd
     * - 0 refers to an empty item stack (air)
     */
    pub fn read_item_stack_net_id_variant(stream: &mut Reader) -> i32 {
        stream.get_var_i32()
    }

    /**
     * This is a union of ItemStackRequestId, LegacyItemStackRequestId, and ServerItemStackId, used in server-bound
     * packets to allow the client to refer to server-known items, or items which may have been modified by a previous
     * as-yet unacknowledged request from the client.
     */
    pub fn write_item_stack_net_id_variant(stream: &mut Writer, id: i32) {
        stream.put_var_i32(id);
    }

    pub fn read_item_stack_request_id(stream: &mut Reader) -> i32 {
        stream.get_var_i32()
    }

    pub fn write_item_stack_request_id(stream: &mut Writer, id: i32) {
        stream.put_var_i32(id);
    }

    pub fn read_legacy_item_stack_request_id(stream: &mut Reader) -> i32 {
        stream.get_var_i32()
    }

    pub fn write_legacy_item_stack_request_id(stream: &mut Writer, id: i32) {
        stream.put_var_i32(id);
    }

    pub fn read_server_item_stack_id(stream: &mut Reader) -> i32 {
        stream.get_var_i32()
    }

    pub fn write_server_item_stack_id(stream: &mut Writer, id: i32) {
        stream.put_var_i32(id);
    }

    fn get_item_stack_footer(stream: &mut Reader, id: i32, meta: u32, count: u16) -> ItemStack {
        let block_runtime_id = stream.get_var_i32();
        let raw_extra_data = PacketSerializer::get_byte_string(stream);

        ItemStack::new(id, meta, count, block_runtime_id, raw_extra_data)
    }

    fn put_item_stack_footer(stream: &mut Writer, stack: &ItemStack) {
        stream.put_var_i32(stack.block_runtime_id);
        Self::put_byte_string(stream, &stack.raw_extra_data);
    }

    pub fn get_item_stack_without_stack_id(stream: &mut Reader) -> ItemStack {
        let id = stream.get_var_i32();
        let count = stream.get_u16_le();
        let meta = stream.get_var_u32();

        Self::get_item_stack_footer(stream, id, meta, count)
    }

    pub fn put_item_stack_without_stack_id(stream: &mut Writer, stack: &ItemStack) {
        stream.put_var_i32(stack.id);
        stream.put_u16_le(stack.count);
        stream.put_var_u32(stack.meta);
        Self::put_item_stack_footer(stream, stack);
    }

    pub fn get_network_item_stack_descriptor(stream: &mut Reader) -> ItemStackWrapper {
        let id = stream.get_i16_le();
        let count = stream.get_u16_le();
        let meta = stream.get_var_u32();
        let has_net_id = stream.get_bool();
        let mut stack_id = 0;
        if has_net_id {
            stack_id = stream.get_var_i32();
        }
        let block_runtime_id = stream.get_var_u32();
        let raw_extra_data = PacketSerializer::get_byte_string(stream);

        ItemStackWrapper { stack_id, item_stack: ItemStack {
            id: id as i32,
            meta,
            count,
            block_runtime_id: block_runtime_id as i32,
            raw_extra_data,
        } }
    }

    pub fn put_network_item_stack_descriptor(stream: &mut Writer, wrapper: &ItemStackWrapper) {
        stream.put_i16_le(wrapper.item_stack.id as i16);
        stream.put_u16_le(wrapper.item_stack.count);
        stream.put_var_u32(wrapper.item_stack.meta);
        let has_net_id = wrapper.stack_id != 0;
        stream.put_bool(has_net_id);
        if has_net_id {
            stream.put_var_i32(wrapper.stack_id);
        }
        stream.put_var_u32(wrapper.item_stack.block_runtime_id as u32);
        PacketSerializer::put_byte_string(stream, &wrapper.item_stack.raw_extra_data);
    }

    pub fn get_recipe_ingredient(stream: &mut Reader) -> RecipeIngredient {
        let descriptor_type = stream.get_var_u32();
        let _ = stream.get_u8();
        let descriptor = match descriptor_type {
            ItemDescriptorType::DEFAULT => Some(ItemDescriptor::Default(DefaultItemDescriptor::read(stream))),
            ItemDescriptorType::TAG => Some(ItemDescriptor::Tag(TagItemDescriptor::read(stream))),
            ItemDescriptorType::MOLANG => Some(ItemDescriptor::Molang(MolangItemDescriptor::read(stream))),
            _ => None,
        };
        let count = stream.get_i16_le();

        RecipeIngredient { descriptor, count: count as i32 }
    }

    pub fn put_recipe_ingredient(stream: &mut Writer, ingredient: &RecipeIngredient) {
        if let Some(descriptor) = &ingredient.descriptor {
            stream.put_var_u32(descriptor.type_id());
            stream.put_u8(descriptor.type_id() as u8);
            descriptor.write(stream);
        } else {
            stream.put_var_u32(0);
            stream.put_u8(0);
        }
        stream.put_i16_le(ingredient.count as i16);
    }

    fn read_game_rule(
        stream: &mut Reader,
        rule_type: u32,
        is_player_modifiable: bool,
        is_start_game: bool,
    ) -> GameRule {
        match rule_type {
            GameRuleTypes::BOOL => GameRule::Bool(BoolGameRule::read(stream, is_player_modifiable)),
            GameRuleTypes::INT => GameRule::Int(IntGameRule::read(
                stream,
                is_player_modifiable,
                is_start_game,
            )),
            GameRuleTypes::FLOAT => {
                GameRule::Float(FloatGameRule::read(stream, is_player_modifiable))
            }
            _ => {
                panic!("Unknown game rule type: {}", rule_type);
            }
        }
    }

    pub fn get_game_rules(stream: &mut Reader, is_start_game: bool) -> HashMap<String, GameRule> {
        let count = stream.get_var_u32() as usize;
        let mut rules = HashMap::new();
        for _ in 0..count {
            let name = PacketSerializer::get_string(stream);
            let is_player_modifiable = stream.get_bool();
            let rule_type = stream.get_var_u32();
            rules.insert(name, Self::read_game_rule(stream, rule_type, is_player_modifiable, is_start_game));
        }
        rules
    }

    pub fn put_game_rules(stream: &mut Writer, rules: &mut HashMap<String, GameRule>, is_start_game: bool) {
        stream.put_var_u32(rules.len() as u32);
        for (name, rule) in rules {
            PacketSerializer::put_string(stream, name);
            stream.put_bool(rule.is_player_modifiable());
            stream.put_var_u32(rule.id());
            rule.write(stream, is_start_game);
        }
    }

    pub fn get_command_origin_data(stream: &mut Reader) -> CommandOriginData {
        let origin_type = PacketSerializer::get_string(stream);
        let uuid = PacketSerializer::get_uuid(stream);
        let request_id = PacketSerializer::get_string(stream);
        let player_actor_unique_id = stream.get_i64_le();

        CommandOriginData {
            origin_type,
            uuid,
            request_id,
            player_actor_unique_id,
        }
    }

    pub fn put_command_origin_data(stream: &mut Writer, data: &CommandOriginData) {
        PacketSerializer::put_string(stream, &data.origin_type);
        PacketSerializer::put_uuid(stream, &data.uuid);
        PacketSerializer::put_string(stream, &data.request_id);
        stream.put_i64_le(data.player_actor_unique_id);
    }

    pub fn get_skin(stream: &mut Reader) -> SkinData {
        let skin_id = PacketSerializer::get_string(stream);
        let play_fab_id = PacketSerializer::get_string(stream);
        let resource_patch = PacketSerializer::get_string(stream);
        let skin_image = PacketSerializer::get_skin_image(stream);
        let animation_count = stream.get_var_u32();
        let mut animations = Vec::with_capacity(animation_count as usize);
        for _ in 0..animation_count {
            let skin_image = PacketSerializer::get_skin_image(stream);
            let animation_type = stream.get_var_u32();
            let animation_frames = stream.get_f32_le();
            let expression_type = stream.get_var_u32();
            animations.push(SkinAnimation::new(
                skin_image,
                animation_type,
                animation_frames,
                expression_type,
            ));
        }
        let cape_image = Some(PacketSerializer::get_skin_image(stream));
        let geometry_data = PacketSerializer::get_string(stream);
        let geometry_data_engine_version = PacketSerializer::get_string(stream);
        let animation_data = PacketSerializer::get_string(stream);
        let cape_id = PacketSerializer::get_string(stream);
        let full_skin_id = Option::from(PacketSerializer::get_string(stream));
        let arm_size = stream.get_u8();
        let skin_color = stream.get_i32_le();
        let persona_piece_count = stream.get_var_u32();
        let mut persona_pieces = Vec::with_capacity(persona_piece_count as usize);
        for _ in 0..persona_piece_count {
            let piece_id = PacketSerializer::get_string(stream);
            let piece_type = stream.get_i32_le();
            let pack_id = PacketSerializer::get_uuid(stream);
            let is_default_piece = stream.get_bool();
            let product_id = PacketSerializer::get_string(stream);
            persona_pieces.push(PersonaSkinPiece::new(
                piece_id,
                piece_type,
                pack_id,
                is_default_piece,
                product_id,
            ))
        }
        let piece_tint_color_count = stream.get_var_u32();
        let mut piece_tint_colors = Vec::with_capacity(piece_tint_color_count as usize);
        for _ in 0..piece_tint_color_count {
            let piece_type = PacketSerializer::get_string(stream);
            let mut colors = Vec::with_capacity(PersonaPieceTintColor::COLOR_COUNT as usize);
            for _ in 0..PersonaPieceTintColor::COLOR_COUNT {
                colors.push(stream.get_i32_le());
            }
            piece_tint_colors.push(PersonaPieceTintColor::new(piece_type, colors));
        }
        let premium = stream.get_bool();
        let persona = stream.get_bool();
        let persona_cape_on_classic = stream.get_bool();
        let is_primary_user = stream.get_bool();
        let is_override = stream.get_bool();
        let trusted_skin_flag = PacketSerializer::get_string(stream);
        let profile_hash = PacketSerializer::get_string(stream);

        SkinData {
            skin_id,
            play_fab_id,
            resource_patch,
            skin_image,
            animations,
            cape_image,
            geometry_data,
            geometry_data_engine_version,
            animation_data,
            cape_id,
            full_skin_id,
            arm_size,
            skin_color,
            persona_pieces,
            piece_tint_colors,
            is_verified: true,
            premium,
            persona,
            persona_cape_on_classic,
            is_primary_user,
            is_override,
            trusted_skin_flag,
            profile_hash
        }
    }

    pub fn put_skin(stream: &mut Writer, skin: &SkinData) {
        PacketSerializer::put_string(stream, &skin.skin_id);
        PacketSerializer::put_string(stream, &skin.play_fab_id);
        PacketSerializer::put_string(stream, &skin.resource_patch);
        PacketSerializer::put_skin_image(stream, &skin.skin_image);
        stream.put_var_u32(skin.animations.len() as u32);
        for animation in skin.animations.iter() {
            Self::put_skin_image(stream, animation.image());
            stream.put_var_u32(animation.animation_type());
            stream.put_f32_le(animation.frames());
            stream.put_var_u32(animation.expression_type());
        }
        if let Some(cape) = skin.cape_image.as_ref() {
            Self::put_skin_image(stream, cape);
        }
        PacketSerializer::put_string(stream, &skin.geometry_data);
        PacketSerializer::put_string(stream, &skin.geometry_data_engine_version);
        PacketSerializer::put_string(stream, &skin.animation_data);
        PacketSerializer::put_string(stream, &skin.cape_id);
        if let Some(full_skin_id) = skin.full_skin_id.as_ref() {
            PacketSerializer::put_string(stream, full_skin_id);
        }
        stream.put_u8(skin.arm_size);
        stream.put_i32_le(skin.skin_color);
        stream.put_var_u32(skin.persona_pieces.len() as u32);
        for piece in skin.persona_pieces.iter() {
            PacketSerializer::put_string(stream, &piece.piece_id());
            stream.put_i32_le(piece.piece_type());
            PacketSerializer::put_uuid(stream, &piece.pack_id());
            stream.put_bool(piece.is_default_piece());
            PacketSerializer::put_string(stream, &piece.product_id());
        }
        stream.put_var_u32(skin.piece_tint_colors.len() as u32);
        for piece_tint_color in skin.piece_tint_colors.iter() {
            PacketSerializer::put_string(stream, &piece_tint_color.piece_type());
            for color in piece_tint_color.colors().iter() {
                stream.put_i32_le(*color);
            }
        }
        stream.put_bool(skin.premium);
        stream.put_bool(skin.persona);
        stream.put_bool(skin.persona_cape_on_classic);
        stream.put_bool(skin.is_primary_user);
        stream.put_bool(skin.is_override);
        PacketSerializer::put_string(stream, &skin.trusted_skin_flag);
        PacketSerializer::put_string(stream, &skin.profile_hash);
    }

    fn get_skin_image(stream: &mut Reader) -> SkinImage {
        let width = stream.get_u32_le();
        let height = stream.get_u32_le();

        // check later (improve get string func)
        let length = stream.get_var_u32();
        let data = stream.get(length as usize).to_vec();
        //let data = PacketSerializer::get_string(stream);

        SkinImage::new(width, height, data)
    }

    fn put_skin_image(stream: &mut Writer, skin_image: &SkinImage) {
        stream.put_u32_le(skin_image.width());
        stream.put_u32_le(skin_image.height());
        // check later (improve get string func)
        stream.put_var_u32(skin_image.data().len() as u32);
        stream.put(skin_image.data());
        //PacketSerializer::put_string(stream, skin_image.data());
    }

    pub fn get_structure_settings(stream: &mut Reader) -> StructureSettings {
        let palette_name = PacketSerializer::get_string(stream);
        let ignore_entities = stream.get_bool();
        let ignore_blocks = stream.get_bool();
        let allow_non_ticking_chunks = stream.get_bool();
        let dimensions = PacketSerializer::get_block_pos(stream);
        let offset = PacketSerializer::get_block_pos(stream);
        let last_touched_by_player_id = PacketSerializer::get_actor_unique_id(stream);
        let rotation = stream.get_u8();
        let mirror = stream.get_u8();
        let animation_mode = stream.get_u8();
        let animation_seconds = stream.get_f32_le();
        let integrity_value = stream.get_f32_le();
        let integrity_seed = stream.get_u32_le();
        let pivot = PacketSerializer::get_vector3(stream);

        StructureSettings {
            palette_name,
            ignore_entities,
            ignore_blocks,
            allow_non_ticking_chunks,
            dimensions,
            offset,
            last_touched_by_player_id,
            rotation,
            mirror,
            animation_mode,
            animation_seconds,
            integrity_value,
            integrity_seed,
            pivot,
        }
    }

    pub fn put_structure_settings(stream: &mut Writer, structure_settings: &StructureSettings) {
        PacketSerializer::put_string(stream, &structure_settings.palette_name);
        stream.put_bool(structure_settings.ignore_entities);
        stream.put_bool(structure_settings.ignore_blocks);
        stream.put_bool(structure_settings.allow_non_ticking_chunks);
        PacketSerializer::put_block_pos(stream, &structure_settings.dimensions);
        PacketSerializer::put_block_pos(stream, &structure_settings.offset);
        PacketSerializer::put_actor_unique_id(stream, structure_settings.last_touched_by_player_id);
        stream.put_u8(structure_settings.rotation);
        stream.put_u8(structure_settings.mirror);
        stream.put_u8(structure_settings.animation_mode);
        stream.put_f32_le(structure_settings.animation_seconds);
        stream.put_f32_le(structure_settings.integrity_value);
        stream.put_u32_le(structure_settings.integrity_seed);
        PacketSerializer::put_vector3(stream, &structure_settings.pivot);
    }

    pub fn get_structure_editor_data(stream: &mut Reader) -> StructureEditorData {
        let structure_name = PacketSerializer::get_string(stream);
        let filtered_structure_name = PacketSerializer::get_string(stream);
        let structure_data_field = PacketSerializer::get_string(stream);
        let include_players = stream.get_bool();
        let show_bounding_box = stream.get_bool();
        let structure_block_type = stream.get_var_i32();
        let structure_settings = PacketSerializer::get_structure_settings(stream);
        let structure_redstone_save_mode = stream.get_u8();

        StructureEditorData {
            structure_name,
            filtered_structure_name,
            structure_data_field,
            include_players,
            show_bounding_box,
            structure_block_type,
            structure_settings,
            structure_redstone_save_mode,
        }
    }

    pub fn put_structure_editor_data(stream: &mut Writer, structure_editor_data: &StructureEditorData) {
        PacketSerializer::put_string(stream, &structure_editor_data.structure_name);
        PacketSerializer::put_string(stream, &structure_editor_data.filtered_structure_name);
        PacketSerializer::put_string(stream, &structure_editor_data.structure_data_field);
        stream.put_bool(structure_editor_data.include_players);
        stream.put_bool(structure_editor_data.show_bounding_box);
        stream.put_var_i32(structure_editor_data.structure_block_type);
        PacketSerializer::put_structure_settings(stream, &structure_editor_data.structure_settings);
        stream.put_u8(structure_editor_data.structure_redstone_save_mode);
    }

    pub fn read_optional<T, F>(stream: &mut Reader, read_fn: F) -> Option<T>
    where
        F: FnOnce(&mut Reader) -> T,
    {
        let optional = stream.get_bool();
        if optional {
            Some(read_fn(stream))
        } else {
            None
        }
    }

    pub fn write_optional<T, F>(stream: &mut Writer, value: &Option<T>, write_fn: F)
    where
        F: FnOnce(&mut Writer, &T),
    {
        if let Some(v) = value {
            stream.put_bool(true);
            write_fn(stream, v);
        } else {
            stream.put_bool(false);
        }
    }

    pub fn read_double_optional<T, F>(stream: &mut Reader, read_fn: F) -> Option<T>
    where
        F: FnOnce(&mut Reader) -> T,
    {
        let outer = stream.get_bool();
        if outer {
            Self::read_optional(stream, read_fn)
        } else {
            None
        }
    }

    pub fn write_double_optional<T, F>(stream: &mut Writer, value: &Option<T>, write_fn: F)
    where
        F: FnOnce(&mut Writer, &T),
    {
        stream.put_bool(true);
        Self::write_optional(stream, value, write_fn);
    }
}
