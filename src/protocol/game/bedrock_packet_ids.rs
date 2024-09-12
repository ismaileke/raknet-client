#[repr(u16)]
pub enum BedrockPacketType {
    Login,
    PlayStatus,
    ServerToClientHandshake,
    ClientToServerHandshake,
    Disconnect,
    ResourcePacksInfo,
    ResourcePackStack,
    ResourcePackClientResponse,
    TextPacket,
    SetTime,
    StartGame,
    AddPlayer,
    AddActor,
    RemoveActor,
    AddItemActor,
    ServerPlayerPostMovePosition,
    TakeItemActor,
    MoveActorAbsolute,
    MovePlayer,
    PassengerJump,
    UpdateBlock,
    AddPainting,
    LevelSoundEventV1,
    LevelEvent,
    BlockEvent,
    ActorEvent,
    MobEffect,
    UpdateAttributes,
    InventoryTransaction,
    MobEquipment,
    MobArmorEquipment,
    Interact,
    BlockPickRequest,
    ActorPickRequest,
    PlayerAction,
    HurtArmor,
    SetActorData,
    SetActorMotion,
    SetActorLink,
    SetHealth,
    SetSpawnPosition,
    Animate,
    Respawn,
    ContainerOpen,
    ContainerClose,
    PlayerHotbar,
    InventoryContent,
    InventorySlot,
    ContainerSetData,
    CraftingData,
    GUIDataPackItem,
    BlockActorData,
    PlayerInput,
    LevelChunk,
    SetCommandsEnabled,
    SetDifficulty,
    ChangeDimension,
    SetPlayerGameType,
    PlayerList,
    SimpleEvent,
    LegacyTelemetryEvent,
    SpawnExperienceOrb,
    ClientboundMapItemData,
    MapInfoRequest,
    RequestChunkRadius,
    ChunkRadiusUpdated,
    GameRulesChanged,
    Camera,
    BossEvent,
    ShowCredits,
    AvailableCommands,
    CommandRequest,
    CommandBlockUpdate,
    CommandOutput,
    UpdateTrade,
    UpdateEquip,
    ResourcePackDataInfo,
    ResourcePackChunkData,
    ResourcePackChunkRequest,
    Transfer,
    PlaySound,
    StopSound,
    SetTitle,
    AddBehaviorTree,
    StructureBlockUpdate,
    ShowStoreOffer,
    PurchaseReceipt,
    PlayerSkin,
    SubClientLogin,
    AutomationClientConnect,
    SetLastHurtBy,
    BookEdit,
    NPCRequest,
    PhotoTransfer,
    ModalFormRequest,
    ModalFormResponse,
    ServerSettingsRequest,
    ServerSettingsResponse,
    ShowProfile,
    SetDefaultGameType,
    RemoveObjective,
    SetDisplayObjective,
    SetScore,
    LabTable,
    UpdateBlockSynced,
    MoveActorDelta,
    SetScoreboardIdentity,
    SetLocalPlayerAsInitialized,
    UpdateSoftEnum,
    NetworkStackLatency,
    SpawnParticleEvent,
    AvailableActorIdentifiers,
    LevelSoundEventV2,
    NetworkChunkPublisherUpdate,
    BiomeDefinitionList,
    LevelSoundEvent,
    LevelEventGeneric,
    LecternUpdate,
    ClientCacheStatus,
    OnScreenTextureAnimation,
    MapCreateLockedCopy,
    StructureTemplateDataRequest,
    StructureTemplateDataResponse,
    ClientCacheBlobStatus,
    ClientCacheMissResponse,
    EducationSettings,
    Emote,
    MultiplayerSettings,
    SettingsCommand,
    AnvilDamage,
    CompletedUsingItem,
    NetworkSettings,
    PlayerAuthInput,
    CreativeContent,
    PlayerEnchantOptions,
    ItemStackRequest,
    ItemStackResponse,
    PlayerArmorDamage,
    CodeBuilder,
    UpdatePlayerGameType,
    EmoteList,
    PositionTrackingDBServerBroadcast,
    PositionTrackingDBClientRequest,
    DebugInfo,
    PacketViolationWarning,
    MotionPredictionHints,
    AnimateEntity,
    CameraShake,
    PlayerFog,
    CorrectPlayerMovePrediction,
    ItemComponent,
    ClientboundDebugRenderer,
    SyncActorProperty,
    AddVolumeEntity,
    RemoveVolumeEntity,
    SimulationType,
    NpcDialogue,
    EduUriResource,
    CreatePhoto,
    UpdateSubChunkBlocks,
    SubChunk,
    SubChunkRequest,
    PlayerStartItemCooldown,
    ScriptMessage,
    CodeBuilderSource,
    TickingAreasLoadStatus,
    DimensionData,
    AgentActionEvent,
    ChangeMobProperty,
    LessonProgress,
    RequestAbility,
    RequestPermissions,
    ToastRequest,
    UpdateAbilities,
    UpdateAdventureSettings,
    DeathInfo,
    EditorNetwork,
    FeatureRegistry,
    ServerStats,
    RequestNetworkSettings,
    GameTestRequest,
    GameTestResults,
    UpdateClientInputLocks,
    CameraPresets,
    UnlockedRecipes,
    CameraInstruction,
    CompressedBiomeDefinitionList,
    TrimData,
    OpenSign,
    AgentAnimation,
    RefreshEntitlements,
    PlayerToggleCrafterSlotRequest,
    SetPlayerInventoryOptions,
    SetHud,
    AwardAchievement,
    ClientboundCloseForm,
    ServerboundLoadingScreen,
    JigsawStructureData,
    CurrentStructureFeature,
    ServerboundDiagnostics,
    Unknown
}

impl BedrockPacketType {
    pub(crate) fn from_byte(byte: u16) -> Self {
        match byte {
            0x01 => BedrockPacketType::Login,
            0x02 => BedrockPacketType::PlayStatus,
            0x03 => BedrockPacketType::ServerToClientHandshake,
            0x04 => BedrockPacketType::ClientToServerHandshake,
            0x05 => BedrockPacketType::Disconnect,
            0x06 => BedrockPacketType::ResourcePacksInfo,
            0x07 => BedrockPacketType::ResourcePackStack,
            0x08 => BedrockPacketType::ResourcePackClientResponse,
            0x09 => BedrockPacketType::TextPacket,
            0x0a => BedrockPacketType::SetTime,
            0x0b => BedrockPacketType::StartGame,
            0x0c => BedrockPacketType::AddPlayer,
            0x0d => BedrockPacketType::AddActor,
            0x0e => BedrockPacketType::RemoveActor,
            0x0f => BedrockPacketType::AddItemActor,
            0x10 => BedrockPacketType::ServerPlayerPostMovePosition,
            0x11 => BedrockPacketType::TakeItemActor,
            0x12 => BedrockPacketType::MoveActorAbsolute,
            0x13 => BedrockPacketType::MovePlayer,
            0x14 => BedrockPacketType::PassengerJump,
            0x15 => BedrockPacketType::UpdateBlock,
            0x16 => BedrockPacketType::AddPainting,
            0x18 => BedrockPacketType::LevelSoundEventV1,
            0x19 => BedrockPacketType::LevelEvent,
            0x1a => BedrockPacketType::BlockEvent,
            0x1b => BedrockPacketType::ActorEvent,
            0x1c => BedrockPacketType::MobEffect,
            0x1d => BedrockPacketType::UpdateAttributes,
            0x1e => BedrockPacketType::InventoryTransaction,
            0x1f => BedrockPacketType::MobEquipment,
            0x20 => BedrockPacketType::MobArmorEquipment,
            0x21 => BedrockPacketType::Interact,
            0x22 => BedrockPacketType::BlockPickRequest,
            0x23 => BedrockPacketType::ActorPickRequest,
            0x24 => BedrockPacketType::PlayerAction,
            0x26 => BedrockPacketType::HurtArmor,
            0x27 => BedrockPacketType::SetActorData,
            0x28 => BedrockPacketType::SetActorMotion,
            0x29 => BedrockPacketType::SetActorLink,
            0x2a => BedrockPacketType::SetHealth,
            0x2b => BedrockPacketType::SetSpawnPosition,
            0x2c => BedrockPacketType::Animate,
            0x2d => BedrockPacketType::Respawn,
            0x2e => BedrockPacketType::ContainerOpen,
            0x2f => BedrockPacketType::ContainerClose,
            0x30 => BedrockPacketType::PlayerHotbar,
            0x31 => BedrockPacketType::InventoryContent,
            0x32 => BedrockPacketType::InventorySlot,
            0x33 => BedrockPacketType::ContainerSetData,
            0x34 => BedrockPacketType::CraftingData,
            0x36 => BedrockPacketType::GUIDataPackItem,
            0x38 => BedrockPacketType::BlockActorData,
            0x39 => BedrockPacketType::PlayerInput,
            0x3a => BedrockPacketType::LevelChunk,
            0x3b => BedrockPacketType::SetCommandsEnabled,
            0x3c => BedrockPacketType::SetDifficulty,
            0x3d => BedrockPacketType::ChangeDimension,
            0x3e => BedrockPacketType::SetPlayerGameType,
            0x3f => BedrockPacketType::PlayerList,
            0x40 => BedrockPacketType::SimpleEvent,
            0x41 => BedrockPacketType::LegacyTelemetryEvent,
            0x42 => BedrockPacketType::SpawnExperienceOrb,
            0x43 => BedrockPacketType::ClientboundMapItemData,
            0x44 => BedrockPacketType::MapInfoRequest,
            0x45 => BedrockPacketType::RequestChunkRadius,
            0x46 => BedrockPacketType::ChunkRadiusUpdated,
            0x48 => BedrockPacketType::GameRulesChanged,
            0x49 => BedrockPacketType::Camera,
            0x4a => BedrockPacketType::BossEvent,
            0x4b => BedrockPacketType::ShowCredits,
            0x4c => BedrockPacketType::AvailableCommands,
            0x4d => BedrockPacketType::CommandRequest,
            0x4e => BedrockPacketType::CommandBlockUpdate,
            0x4f => BedrockPacketType::CommandOutput,
            0x50 => BedrockPacketType::UpdateTrade,
            0x51 => BedrockPacketType::UpdateEquip,
            0x52 => BedrockPacketType::ResourcePackDataInfo,
            0x53 => BedrockPacketType::ResourcePackChunkData,
            0x54 => BedrockPacketType::ResourcePackChunkRequest,
            0x55 => BedrockPacketType::Transfer,
            0x56 => BedrockPacketType::PlaySound,
            0x57 => BedrockPacketType::StopSound,
            0x58 => BedrockPacketType::SetTitle,
            0x59 => BedrockPacketType::AddBehaviorTree,
            0x5a => BedrockPacketType::StructureBlockUpdate,
            0x5b => BedrockPacketType::ShowStoreOffer,
            0x5c => BedrockPacketType::PurchaseReceipt,
            0x5d => BedrockPacketType::PlayerSkin,
            0x5e => BedrockPacketType::SubClientLogin,
            0x5f => BedrockPacketType::AutomationClientConnect,
            0x60 => BedrockPacketType::SetLastHurtBy,
            0x61 => BedrockPacketType::BookEdit,
            0x62 => BedrockPacketType::NPCRequest,
            0x63 => BedrockPacketType::PhotoTransfer,
            0x64 => BedrockPacketType::ModalFormRequest,
            0x65 => BedrockPacketType::ModalFormResponse,
            0x66 => BedrockPacketType::ServerSettingsRequest,
            0x67 => BedrockPacketType::ServerSettingsResponse,
            0x68 => BedrockPacketType::ShowProfile,
            0x69 => BedrockPacketType::SetDefaultGameType,
            0x6a => BedrockPacketType::RemoveObjective,
            0x6b => BedrockPacketType::SetDisplayObjective,
            0x6c => BedrockPacketType::SetScore,
            0x6d => BedrockPacketType::LabTable,
            0x6e => BedrockPacketType::UpdateBlockSynced,
            0x6f => BedrockPacketType::MoveActorDelta,
            0x70 => BedrockPacketType::SetScoreboardIdentity,
            0x71 => BedrockPacketType::SetLocalPlayerAsInitialized,
            0x72 => BedrockPacketType::UpdateSoftEnum,
            0x73 => BedrockPacketType::NetworkStackLatency,
            0x76 => BedrockPacketType::SpawnParticleEvent,
            0x77 => BedrockPacketType::AvailableActorIdentifiers,
            0x78 => BedrockPacketType::LevelSoundEventV2,
            0x79 => BedrockPacketType::NetworkChunkPublisherUpdate,
            0x7a => BedrockPacketType::BiomeDefinitionList,
            0x7b => BedrockPacketType::LevelSoundEvent,
            0x7c => BedrockPacketType::LevelEventGeneric,
            0x7d => BedrockPacketType::LecternUpdate,
            0x81 => BedrockPacketType::ClientCacheStatus,
            0x82 => BedrockPacketType::OnScreenTextureAnimation,
            0x83 => BedrockPacketType::MapCreateLockedCopy,
            0x84 => BedrockPacketType::StructureTemplateDataRequest,
            0x85 => BedrockPacketType::StructureTemplateDataResponse,
            0x87 => BedrockPacketType::ClientCacheBlobStatus,
            0x88 => BedrockPacketType::ClientCacheMissResponse,
            0x89 => BedrockPacketType::EducationSettings,
            0x8a => BedrockPacketType::Emote,
            0x8b => BedrockPacketType::MultiplayerSettings,
            0x8c => BedrockPacketType::SettingsCommand,
            0x8d => BedrockPacketType::AnvilDamage,
            0x8e => BedrockPacketType::CompletedUsingItem,
            0x8f => BedrockPacketType::NetworkSettings,
            0x90 => BedrockPacketType::PlayerAuthInput,
            0x91 => BedrockPacketType::CreativeContent,
            0x92 => BedrockPacketType::PlayerEnchantOptions,
            0x93 => BedrockPacketType::ItemStackRequest,
            0x94 => BedrockPacketType::ItemStackResponse,
            0x95 => BedrockPacketType::PlayerArmorDamage,
            0x96 => BedrockPacketType::CodeBuilder,
            0x97 => BedrockPacketType::UpdatePlayerGameType,
            0x98 => BedrockPacketType::EmoteList,
            0x99 => BedrockPacketType::PositionTrackingDBServerBroadcast,
            0x9a => BedrockPacketType::PositionTrackingDBClientRequest,
            0x9b => BedrockPacketType::DebugInfo,
            0x9c => BedrockPacketType::PacketViolationWarning,
            0x9d => BedrockPacketType::MotionPredictionHints,
            0x9e => BedrockPacketType::AnimateEntity,
            0x9f => BedrockPacketType::CameraShake,
            0xa0 => BedrockPacketType::PlayerFog,
            0xa1 => BedrockPacketType::CorrectPlayerMovePrediction,
            0xa2 => BedrockPacketType::ItemComponent,
            0xa4 => BedrockPacketType::ClientboundDebugRenderer,
            0xa5 => BedrockPacketType::SyncActorProperty,
            0xa6 => BedrockPacketType::AddVolumeEntity,
            0xa7 => BedrockPacketType::RemoveVolumeEntity,
            0xa8 => BedrockPacketType::SimulationType,
            0xa9 => BedrockPacketType::NpcDialogue,
            0xaa => BedrockPacketType::EduUriResource,
            0xab => BedrockPacketType::CreatePhoto,
            0xac => BedrockPacketType::UpdateSubChunkBlocks,
            0xae => BedrockPacketType::SubChunk,
            0xaf => BedrockPacketType::SubChunkRequest,
            0xb0 => BedrockPacketType::PlayerStartItemCooldown,
            0xb1 => BedrockPacketType::ScriptMessage,
            0xb2 => BedrockPacketType::CodeBuilderSource,
            0xb3 => BedrockPacketType::TickingAreasLoadStatus,
            0xb4 => BedrockPacketType::DimensionData,
            0xb5 => BedrockPacketType::AgentActionEvent,
            0xb6 => BedrockPacketType::ChangeMobProperty,
            0xb7 => BedrockPacketType::LessonProgress,
            0xb8 => BedrockPacketType::RequestAbility,
            0xb9 => BedrockPacketType::RequestPermissions,
            0xba => BedrockPacketType::ToastRequest,
            0xbb => BedrockPacketType::UpdateAbilities,
            0xbc => BedrockPacketType::UpdateAdventureSettings,
            0xbd => BedrockPacketType::DeathInfo,
            0xbe => BedrockPacketType::EditorNetwork,
            0xbf => BedrockPacketType::FeatureRegistry,
            0xc0 => BedrockPacketType::ServerStats,
            0xc1 => BedrockPacketType::RequestNetworkSettings,
            0xc2 => BedrockPacketType::GameTestRequest,
            0xc3 => BedrockPacketType::GameTestResults,
            0xc4 => BedrockPacketType::UpdateClientInputLocks,
            0xc6 => BedrockPacketType::CameraPresets,
            0xc7 => BedrockPacketType::UnlockedRecipes,
            0x12c => BedrockPacketType::CameraInstruction,
            0x12d => BedrockPacketType::CompressedBiomeDefinitionList,
            0x12e => BedrockPacketType::TrimData,
            0x12f => BedrockPacketType::OpenSign,
            0x130 => BedrockPacketType::AgentAnimation,
            0x131 => BedrockPacketType::RefreshEntitlements,
            0x132 => BedrockPacketType::PlayerToggleCrafterSlotRequest,
            0x133 => BedrockPacketType::SetPlayerInventoryOptions,
            0x134 => BedrockPacketType::SetHud,
            0x135 => BedrockPacketType::AwardAchievement,
            0x136 => BedrockPacketType::ClientboundCloseForm,
            0x138 => BedrockPacketType::ServerboundLoadingScreen,
            0x139 => BedrockPacketType::JigsawStructureData,
            0x13a => BedrockPacketType::CurrentStructureFeature,
            0x13b => BedrockPacketType::ServerboundDiagnostics,
            _ => BedrockPacketType::Unknown,
        }
    }
    pub(crate) fn get_byte(self) -> u16 {
        match self {
            BedrockPacketType::Login => 0x01,
            BedrockPacketType::PlayStatus => 0x02,
            BedrockPacketType::ServerToClientHandshake => 0x03,
            BedrockPacketType::ClientToServerHandshake => 0x04,
            BedrockPacketType::Disconnect => 0x05,
            BedrockPacketType::ResourcePacksInfo => 0x06,
            BedrockPacketType::ResourcePackStack => 0x07,
            BedrockPacketType::ResourcePackClientResponse => 0x08,
            BedrockPacketType::TextPacket => 0x09,
            BedrockPacketType::SetTime => 0x0a,
            BedrockPacketType::StartGame => 0x0b,
            BedrockPacketType::AddPlayer => 0x0c,
            BedrockPacketType::AddActor => 0x0d,
            BedrockPacketType::RemoveActor => 0x0e,
            BedrockPacketType::AddItemActor => 0x0f,
            BedrockPacketType::ServerPlayerPostMovePosition => 0x10,
            BedrockPacketType::TakeItemActor => 0x11,
            BedrockPacketType::MoveActorAbsolute => 0x12,
            BedrockPacketType::MovePlayer => 0x13,
            BedrockPacketType::PassengerJump => 0x14,
            BedrockPacketType::UpdateBlock => 0x15,
            BedrockPacketType::AddPainting => 0x16,
            BedrockPacketType::LevelSoundEventV1 => 0x18,
            BedrockPacketType::LevelEvent => 0x19,
            BedrockPacketType::BlockEvent => 0x1a,
            BedrockPacketType::ActorEvent => 0x1b,
            BedrockPacketType::MobEffect => 0x1c,
            BedrockPacketType::UpdateAttributes => 0x1d,
            BedrockPacketType::InventoryTransaction => 0x1e,
            BedrockPacketType::MobEquipment => 0x1f,
            BedrockPacketType::MobArmorEquipment => 0x20,
            BedrockPacketType::Interact => 0x21,
            BedrockPacketType::BlockPickRequest => 0x22,
            BedrockPacketType::ActorPickRequest => 0x23,
            BedrockPacketType::PlayerAction => 0x24,
            BedrockPacketType::HurtArmor => 0x26,
            BedrockPacketType::SetActorData => 0x27,
            BedrockPacketType::SetActorMotion => 0x28,
            BedrockPacketType::SetActorLink => 0x29,
            BedrockPacketType::SetHealth => 0x2a,
            BedrockPacketType::SetSpawnPosition => 0x2b,
            BedrockPacketType::Animate => 0x2c,
            BedrockPacketType::Respawn => 0x2d,
            BedrockPacketType::ContainerOpen => 0x2e,
            BedrockPacketType::ContainerClose => 0x2f,
            BedrockPacketType::PlayerHotbar => 0x30,
            BedrockPacketType::InventoryContent => 0x31,
            BedrockPacketType::InventorySlot => 0x32,
            BedrockPacketType::ContainerSetData => 0x33,
            BedrockPacketType::CraftingData => 0x34,
            BedrockPacketType::GUIDataPackItem => 0x36,
            BedrockPacketType::BlockActorData => 0x38,
            BedrockPacketType::PlayerInput => 0x39,
            BedrockPacketType::LevelChunk => 0x3a,
            BedrockPacketType::SetCommandsEnabled => 0x3b,
            BedrockPacketType::SetDifficulty => 0x3c,
            BedrockPacketType::ChangeDimension => 0x3d,
            BedrockPacketType::SetPlayerGameType => 0x3e,
            BedrockPacketType::PlayerList => 0x3f,
            BedrockPacketType::SimpleEvent => 0x40,
            BedrockPacketType::LegacyTelemetryEvent => 0x41,
            BedrockPacketType::SpawnExperienceOrb => 0x42,
            BedrockPacketType::ClientboundMapItemData => 0x43,
            BedrockPacketType::MapInfoRequest => 0x44,
            BedrockPacketType::RequestChunkRadius => 0x45,
            BedrockPacketType::ChunkRadiusUpdated => 0x46,
            BedrockPacketType::GameRulesChanged => 0x48,
            BedrockPacketType::Camera => 0x49,
            BedrockPacketType::BossEvent => 0x4a,
            BedrockPacketType::ShowCredits => 0x4b,
            BedrockPacketType::AvailableCommands => 0x4c,
            BedrockPacketType::CommandRequest => 0x4d,
            BedrockPacketType::CommandBlockUpdate => 0x4e,
            BedrockPacketType::CommandOutput => 0x4f,
            BedrockPacketType::UpdateTrade => 0x50,
            BedrockPacketType::UpdateEquip => 0x51,
            BedrockPacketType::ResourcePackDataInfo => 0x52,
            BedrockPacketType::ResourcePackChunkData => 0x53,
            BedrockPacketType::ResourcePackChunkRequest => 0x54,
            BedrockPacketType::Transfer => 0x55,
            BedrockPacketType::PlaySound => 0x56,
            BedrockPacketType::StopSound => 0x57,
            BedrockPacketType::SetTitle => 0x58,
            BedrockPacketType::AddBehaviorTree => 0x59,
            BedrockPacketType::StructureBlockUpdate => 0x5a,
            BedrockPacketType::ShowStoreOffer => 0x5b,
            BedrockPacketType::PurchaseReceipt => 0x5c,
            BedrockPacketType::PlayerSkin => 0x5d,
            BedrockPacketType::SubClientLogin => 0x5e,
            BedrockPacketType::AutomationClientConnect => 0x5f,
            BedrockPacketType::SetLastHurtBy => 0x60,
            BedrockPacketType::BookEdit => 0x61,
            BedrockPacketType::NPCRequest => 0x62,
            BedrockPacketType::PhotoTransfer => 0x63,
            BedrockPacketType::ModalFormRequest => 0x64,
            BedrockPacketType::ModalFormResponse => 0x65,
            BedrockPacketType::ServerSettingsRequest => 0x66,
            BedrockPacketType::ServerSettingsResponse => 0x67,
            BedrockPacketType::ShowProfile => 0x68,
            BedrockPacketType::SetDefaultGameType => 0x69,
            BedrockPacketType::RemoveObjective => 0x6a,
            BedrockPacketType::SetDisplayObjective => 0x6b,
            BedrockPacketType::SetScore => 0x6c,
            BedrockPacketType::LabTable => 0x6d,
            BedrockPacketType::UpdateBlockSynced => 0x6e,
            BedrockPacketType::MoveActorDelta => 0x6f,
            BedrockPacketType::SetScoreboardIdentity => 0x70,
            BedrockPacketType::SetLocalPlayerAsInitialized => 0x71,
            BedrockPacketType::UpdateSoftEnum => 0x72,
            BedrockPacketType::NetworkStackLatency => 0x73,
            BedrockPacketType::SpawnParticleEvent => 0x76,
            BedrockPacketType::AvailableActorIdentifiers => 0x77,
            BedrockPacketType::LevelSoundEventV2 => 0x78,
            BedrockPacketType::NetworkChunkPublisherUpdate => 0x79,
            BedrockPacketType::BiomeDefinitionList => 0x7a,
            BedrockPacketType::LevelSoundEvent => 0x7b,
            BedrockPacketType::LevelEventGeneric => 0x7c,
            BedrockPacketType::LecternUpdate => 0x7d,
            BedrockPacketType::ClientCacheStatus => 0x81,
            BedrockPacketType::OnScreenTextureAnimation => 0x82,
            BedrockPacketType::MapCreateLockedCopy => 0x83,
            BedrockPacketType::StructureTemplateDataRequest => 0x84,
            BedrockPacketType::StructureTemplateDataResponse => 0x85,
            BedrockPacketType::ClientCacheBlobStatus => 0x87,
            BedrockPacketType::ClientCacheMissResponse => 0x88,
            BedrockPacketType::EducationSettings => 0x89,
            BedrockPacketType::Emote => 0x8a,
            BedrockPacketType::MultiplayerSettings => 0x8b,
            BedrockPacketType::SettingsCommand => 0x8c,
            BedrockPacketType::AnvilDamage => 0x8d,
            BedrockPacketType::CompletedUsingItem => 0x8e,
            BedrockPacketType::NetworkSettings => 0x8f,
            BedrockPacketType::PlayerAuthInput => 0x90,
            BedrockPacketType::CreativeContent => 0x91,
            BedrockPacketType::PlayerEnchantOptions => 0x92,
            BedrockPacketType::ItemStackRequest => 0x93,
            BedrockPacketType::ItemStackResponse => 0x94,
            BedrockPacketType::PlayerArmorDamage => 0x95,
            BedrockPacketType::CodeBuilder => 0x96,
            BedrockPacketType::UpdatePlayerGameType => 0x97,
            BedrockPacketType::EmoteList => 0x98,
            BedrockPacketType::PositionTrackingDBServerBroadcast => 0x99,
            BedrockPacketType::PositionTrackingDBClientRequest => 0x9a,
            BedrockPacketType::DebugInfo => 0x9b,
            BedrockPacketType::PacketViolationWarning => 0x9c,
            BedrockPacketType::MotionPredictionHints => 0x9d,
            BedrockPacketType::AnimateEntity => 0x9e,
            BedrockPacketType::CameraShake => 0x9f,
            BedrockPacketType::PlayerFog => 0xa0,
            BedrockPacketType::CorrectPlayerMovePrediction => 0xa1,
            BedrockPacketType::ItemComponent => 0xa2,
            BedrockPacketType::ClientboundDebugRenderer => 0xa4,
            BedrockPacketType::SyncActorProperty => 0xa5,
            BedrockPacketType::AddVolumeEntity => 0xa6,
            BedrockPacketType::RemoveVolumeEntity => 0xa7,
            BedrockPacketType::SimulationType => 0xa8,
            BedrockPacketType::NpcDialogue => 0xa9,
            BedrockPacketType::EduUriResource => 0xaa,
            BedrockPacketType::CreatePhoto => 0xab,
            BedrockPacketType::UpdateSubChunkBlocks => 0xac,
            BedrockPacketType::SubChunk => 0xae,
            BedrockPacketType::SubChunkRequest => 0xaf,
            BedrockPacketType::PlayerStartItemCooldown => 0xb0,
            BedrockPacketType::ScriptMessage => 0xb1,
            BedrockPacketType::CodeBuilderSource => 0xb2,
            BedrockPacketType::TickingAreasLoadStatus => 0xb3,
            BedrockPacketType::DimensionData => 0xb4,
            BedrockPacketType::AgentActionEvent => 0xb5,
            BedrockPacketType::ChangeMobProperty => 0xb6,
            BedrockPacketType::LessonProgress => 0xb7,
            BedrockPacketType::RequestAbility => 0xb8,
            BedrockPacketType::RequestPermissions => 0xb9,
            BedrockPacketType::ToastRequest => 0xba,
            BedrockPacketType::UpdateAbilities => 0xbb,
            BedrockPacketType::UpdateAdventureSettings => 0xbc,
            BedrockPacketType::DeathInfo => 0xbd,
            BedrockPacketType::EditorNetwork => 0xbe,
            BedrockPacketType::FeatureRegistry => 0xbf,
            BedrockPacketType::ServerStats => 0xc0,
            BedrockPacketType::RequestNetworkSettings => 0xc1,
            BedrockPacketType::GameTestRequest => 0xc2,
            BedrockPacketType::GameTestResults => 0xc3,
            BedrockPacketType::UpdateClientInputLocks => 0xc4,
            BedrockPacketType::CameraPresets => 0xc6,
            BedrockPacketType::UnlockedRecipes => 0xc7,
            BedrockPacketType::CameraInstruction => 0x12c,
            BedrockPacketType::CompressedBiomeDefinitionList => 0x12d,
            BedrockPacketType::TrimData => 0x12e,
            BedrockPacketType::OpenSign => 0x12f,
            BedrockPacketType::AgentAnimation => 0x130,
            BedrockPacketType::RefreshEntitlements => 0x131,
            BedrockPacketType::PlayerToggleCrafterSlotRequest => 0x132,
            BedrockPacketType::SetPlayerInventoryOptions => 0x133,
            BedrockPacketType::SetHud => 0x134,
            BedrockPacketType::AwardAchievement => 0x135,
            BedrockPacketType::ClientboundCloseForm => 0x136,
            BedrockPacketType::ServerboundLoadingScreen => 0x138,
            BedrockPacketType::JigsawStructureData => 0x139,
            BedrockPacketType::CurrentStructureFeature => 0x13a,
            BedrockPacketType::ServerboundDiagnostics => 0x13b,
            _ => 0
        }
    }
    pub(crate) fn get_packet_name(id: u16) -> &'static str {
        match id {
            0x01 => "Login",
            0x02 => "Play Status",
            0x03 => "Server To Client Handshake",
            0x04 => "Client To Server Handshake",
            0x05 => "Disconnect",
            0x06 => "Resource Packs Info",
            0x07 => "Resource Pack Stack",
            0x08 => "Resource Pack Client Response",
            0x09 => "Text Packet",
            0x0a => "Set Time",
            0x0b => "Start Game",
            0x0c => "Add Player",
            0x0d => "Add Actor",
            0x0e => "Remove Actor",
            0x0f => "Add Item Actor",
            0x10 => "Server Player Post Move Position",
            0x11 => "Take Item Actor",
            0x12 => "Move Actor Absolute",
            0x13 => "Move Player",
            0x14 => "Passenger Jump",
            0x15 => "Update Block",
            0x16 => "Add Painting",
            0x18 => "Level Sound Event V1",
            0x19 => "Level Event",
            0x1a => "Block Event",
            0x1b => "Actor Event",
            0x1c => "Mob Effect",
            0x1d => "Update Attributes",
            0x1e => "Inventory Transaction",
            0x1f => "Mob Equipment",
            0x20 => "Mob Armor Equipment",
            0x21 => "Interact",
            0x22 => "Block Pick Request",
            0x23 => "Actor Pick Request",
            0x24 => "Player Action",
            0x26 => "Hurt Armor",
            0x27 => "Set Actor Data",
            0x28 => "Set Actor Motion",
            0x29 => "Set Actor Link",
            0x2a => "Set Health",
            0x2b => "Set Spawn Position",
            0x2c => "Animate",
            0x2d => "Respawn",
            0x2e => "Container Open",
            0x2f => "Container Close",
            0x30 => "Player Hotbar",
            0x31 => "Inventory Content",
            0x32 => "Inventory Slot",
            0x33 => "Container Set Data",
            0x34 => "Crafting Data",
            0x36 => "GUI Data Pack Item",
            0x38 => "Block Actor Data",
            0x39 => "Player Input",
            0x3a => "Level Chunk",
            0x3b => "Set Commands Enabled",
            0x3c => "Set Difficulty",
            0x3d => "Change Dimension",
            0x3e => "Set Player Game Type",
            0x3f => "Player List",
            0x40 => "Simple Event",
            0x41 => "Legacy Telemetry Event",
            0x42 => "Spawn Experience Orb",
            0x43 => "Clientbound Map Item Data",
            0x44 => "Map Info Request",
            0x45 => "Request Chunk Radius",
            0x46 => "Chunk Radius Updated",
            0x48 => "Game Rules Changed",
            0x49 => "Camera",
            0x4a => "Boss Event",
            0x4b => "Show Credits",
            0x4c => "Available Commands",
            0x4d => "Command Request",
            0x4e => "Command Block Update",
            0x4f => "Command Output",
            0x50 => "Update Trade",
            0x51 => "Update Equip",
            0x52 => "Resource Pack Data Info",
            0x53 => "Resource Pack Chunk Data",
            0x54 => "Resource Pack Chunk Request",
            0x55 => "Transfer",
            0x56 => "Play Sound",
            0x57 => "Stop Sound",
            0x58 => "Set Title",
            0x59 => "Add Behavior Tree",
            0x5a => "Structure Block Update",
            0x5b => "Show Store Offer",
            0x5c => "Purchase Receipt",
            0x5d => "Player Skin",
            0x5e => "Sub Client Login",
            0x5f => "Automation Client Connect",
            0x60 => "Set Last Hurt By",
            0x61 => "Book Edit",
            0x62 => "NPC Request",
            0x63 => "Photo Transfer",
            0x64 => "Modal Form Request",
            0x65 => "Modal Form Response",
            0x66 => "Server Settings Request",
            0x67 => "Server Settings Response",
            0x68 => "Show Profile",
            0x69 => "Set Default Game Type",
            0x6a => "Remove Objective",
            0x6b => "Set Display Objective",
            0x6c => "Set Score",
            0x6d => "Lab Table",
            0x6e => "Update Block Synced",
            0x6f => "Move Actor Delta",
            0x70 => "Set Scoreboard Identity",
            0x71 => "Set Local Player As Initialized",
            0x72 => "Update Soft Enum",
            0x73 => "Network Stack Latency",
            0x76 => "Spawn Particle Event",
            0x77 => "Available Actor Identifiers",
            0x78 => "Level Sound Event V2",
            0x79 => "Network Chunk Publisher Update",
            0x7a => "Biome Definition List",
            0x7b => "Level Sound Event",
            0x7c => "Level Event Generic",
            0x7d => "Lectern Update",
            0x81 => "Client Cache Status",
            0x82 => "On Screen Texture Animation",
            0x83 => "Map Create Locked Copy",
            0x84 => "Structure Template Data Request",
            0x85 => "Structure Template Data Response",
            0x87 => "Client Cache Blob Status",
            0x88 => "Client Cache Miss Response",
            0x89 => "Education Settings",
            0x8a => "Emote",
            0x8b => "Multiplayer Settings",
            0x8c => "Settings Command",
            0x8d => "Anvil Damage",
            0x8e => "Completed Using Item",
            0x8f => "Network Settings",
            0x90 => "Player Auth Input",
            0x91 => "Creative Content",
            0x92 => "Player Enchant Options",
            0x93 => "Item Stack Request",
            0x94 => "Item Stack Response",
            0x95 => "Player Armor Damage",
            0x96 => "Code Builder",
            0x97 => "Update Player Game Type",
            0x98 => "Emote List",
            0x99 => "Position Tracking DB Server Broadcast",
            0x9a => "Position Tracking DB Client Request",
            0x9b => "Debug Info",
            0x9c => "Packet Violation Warning",
            0x9d => "Motion Prediction Hints",
            0x9e => "Animate Entity",
            0x9f => "Camera Shake",
            0xa0 => "Player Fog",
            0xa1 => "Correct Player Move Prediction",
            0xa2 => "Item Component",
            0xa4 => "Clientbound Debug Renderer",
            0xa5 => "Sync Actor Property",
            0xa6 => "Add Volume Entity",
            0xa7 => "Remove Volume Entity",
            0xa8 => "Simulation Type",
            0xa9 => "Npc Dialogue",
            0xaa => "Edu Uri Resource",
            0xab => "Create Photo",
            0xac => "Update Sub Chunk Blocks",
            0xae => "Sub Chunk",
            0xaf => "Sub Chunk Request",
            0xb0 => "Player Start Item Cooldown",
            0xb1 => "Script Message",
            0xb2 => "Code Builder Source",
            0xb3 => "Ticking Areas Load Status",
            0xb4 => "Dimension Data",
            0xb5 => "Agent Action Event",
            0xb6 => "Change Mob Property",
            0xb7 => "Lesson Progress",
            0xb8 => "Request Ability",
            0xb9 => "Request Permissions",
            0xba => "Toast Request",
            0xbb => "Update Abilities",
            0xbc => "Update Adventure Settings",
            0xbd => "Death Info",
            0xbe => "Editor Network",
            0xbf => "Feature Registry",
            0xc0 => "Server Stats",
            0xc1 => "Request Network Settings",
            0xc2 => "Game Test Request",
            0xc3 => "Game Test Results",
            0xc4 => "Update Client Input Lock",
            _ => "Unknown Packet"
        }
    }
}