use serde::{Deserialize, Serialize};
use serde_json::Map;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "id", rename_all = "snake_case")]
pub enum BlockEntityData {
    #[serde(alias = "minecraft:sign", alias = "minecraft:hanging_sign")]
    Sign(SignBlockEntityData),
    #[serde(rename = "minecraft:banner")]
    Banner(BannerBlockEntityData),
    #[serde(rename = "minecraft:barrel")]
    Barrel(BarrelBlockEntityData),
    #[serde(rename = "minecraft:beacon")]
    Beacon(BeaconBlockEntityData),
    #[serde(rename = "minecraft:bed")]
    Bed(DefaultBlockEntityData),
    #[serde(rename = "minecraft:beehive")]
    Beehive(BeehiveBlockEntityData),
    #[serde(rename = "minecraft:bell")]
    Bell(DefaultBlockEntityData),
    #[serde(rename = "minecraft:blast_furnance")]
    BlastFurnance(BlastFurnanceBlockEntityData),
    #[serde(rename = "minecraft:brewing_stand")]
    BrewingStand(BrewingStandBlockEntityData),
    #[serde(rename = "minecraft:campfire")]
    CampFire(CampFireBlockEntityData),
    #[serde(rename = "minecraft:chiseled_bookshelf")]
    ChiseledBookShelf(ChiseledBookShelfBlockEntityData),
    #[serde(rename = "minecraft:chest")]
    Chest(ChestBlockEntityData),
    #[serde(rename = "minecraft:comparator")]
    Comparator(ComparatorBlockEntityData),
    #[serde(rename = "minecraft:command_block")]
    CommandBlock(CommandBlockEntityData),
    #[serde(rename = "minecraft:conduit")]
    Conduit(ConduitBlockEntityData),
    #[serde(rename = "minecraft:daylight_detector")]
    DayLightDetector(DefaultBlockEntityData),
    #[serde(rename = "minecraft:decorated_pot")]
    DecoratedPot(DefaultBlockEntityData),
    #[serde(rename = "minecraft:dispenser")]
    Dispenser(DispenserBlockEntityData),
    #[serde(rename = "minecraft:dropper")]
    Dropper(DropperBlockEntityData),
    #[serde(rename = "minecraft:enchanting_table")]
    EnchantingTable(DefaultBlockEntityData),
    #[serde(rename = "minecraft:ender_chest")]
    EnderChest(DefaultBlockEntityData),
    #[serde(rename = "minecraft:end_gateway")]
    EndGateway(EndGatewayBlockEntityData),
    #[serde(rename = "minecraft:end_portal")]
    EndPortal(DefaultBlockEntityData),
    #[serde(rename = "minecraft:furnace")]
    Furnace(FurnaceBlockEntityData),
    #[serde(rename = "minecraft:hopper")]
    Hopper(HopperBlockEntityData),
    #[serde(rename = "minecraft:jigsaw")]
    JigSaw(JigSawBlockEntityData),
    #[serde(rename = "minecraft:jukebox")]
    JukeBox(JukeBoxBlockEntityData),
    #[serde(rename = "minecraft:lectern")]
    Lectern(LecternBlockEntityData),
    #[serde(rename = "minecraft:mob_spawner")]
    MobSpawner(DefaultBlockEntityData),
    #[serde(rename = "minecraft:piston")]
    Piston(PistonBlockEntityData),
    #[serde(rename = "minecraft:shulker_box")]
    ShulkerBox(ShulkerBoxBlockEntityData),
    #[serde(rename = "minecraft:skull")]
    Skull(SkullBlockEntityData),
    #[serde(rename = "minecraft:smoker")]
    Smoker(SmokerBlockEntityData),
    #[serde(rename = "minecraft:soul_campfire")]
    SoulCampFire(SoulCampFireBlockEntityData),
    #[serde(rename = "minecraft:structure_block")]
    StructureBlock(Map<String, serde_json::Value>),
    #[serde(rename = "minecraft:trapped_chest")]
    TrappedChest(TrappedChestBlockEntityData),
}

// Implement Struct of each BlockEntityData
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefaultBlockEntityData;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignBlockEntityData {
    pub back_text: SignTextData,
    pub front_text: SignTextData,
    pub is_waxed: i8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignTextData {
    pub color: String,
    pub has_glowing_text: i32,
    pub messages: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BannerBlockEntityData {
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    pub patterns: Vec<Pattern>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pattern {
    pub color: String,
    pub pattern: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BarrelBlockEntityData {
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Lock")]
    pub lock: Option<String>,
    #[serde(rename = "LootTable")]
    pub loot_table: Option<String>,
    #[serde(rename = "LootTableSeed")]
    pub loot_table_seed: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "Count")]
    pub count: i8,
    #[serde(rename = "Slot")]
    pub slot: i8,
    pub id: String,
    pub tag: Map<String, serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeaconBlockEntityData {
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "Lock")]
    pub lock: Option<String>,
    #[serde(rename = "Levels")]
    pub levels: i32,
    #[serde(rename = "Primary")]
    pub primary: i32,
    #[serde(rename = "Secondary")]
    pub secondary: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeehiveBlockEntityData {
    #[serde(rename = "Bees")]
    pub bees: Vec<Map<String, serde_json::Value>>,
    #[serde(rename = "FlowerPos")]
    pub flower_pos: Vec<(i32, i32, i32)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlastFurnanceBlockEntityData {
    #[serde(rename = "BurnTime")]
    pub burn_time: i16,
    #[serde(rename = "CookTime")]
    pub cook_time: i16,
    #[serde(rename = "CookTimeTotal")]
    pub cook_time_total: i16,
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Lock")]
    pub lock: Option<String>,
    #[serde(rename = "RecipesUsed")]
    pub recipes_used: Map<String, serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrewingStandBlockEntityData {
    #[serde(rename = "BrewTime")]
    pub brew_time: i16,
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Lock")]
    pub lock: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampFireBlockEntityData {
    #[serde(rename = "CookingTimes")]
    pub cooking_times: Vec<i32>,
    #[serde(rename = "CookingTotalTimes")]
    pub cooking_total_times: Vec<i32>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChiseledBookShelfBlockEntityData {
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    pub last_interacted_slot: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChestBlockEntityData {
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Lock")]
    pub lock: Option<String>,
    #[serde(rename = "LootTable")]
    pub loot_table: Option<String>,
    #[serde(rename = "LootTableSeed")]
    pub loot_table_seed: Option<i64>,
    pub gold: Option<i8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComparatorBlockEntityData {
    #[serde(rename = "OutputSignal")]
    pub output_signal: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandBlockEntityData {
    pub auto: i8,
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "conditionMet")]
    pub condition_met: i8,
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "LastExecution")]
    pub last_execution: i64,
    #[serde(rename = "LastOutput")]
    pub last_output: String,
    pub powered: i8,
    #[serde(rename = "SuccessCount")]
    pub success_count: i32,
    #[serde(rename = "TrackOutput")]
    pub track_output: i8,
    #[serde(rename = "UpdateLastExecution")]
    pub update_last_execution: i8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConduitBlockEntityData {
    #[serde(rename = "Target")]
    pub target: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DispenserBlockEntityData {
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Lock")]
    pub lock: Option<String>,
    #[serde(rename = "LootTable")]
    pub loot_table: Option<String>,
    #[serde(rename = "LootTableSeed")]
    pub loot_table_seed: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DropperBlockEntityData {
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Lock")]
    pub lock: Option<String>,
    #[serde(rename = "LootTable")]
    pub loot_table: Option<String>,
    #[serde(rename = "LootTableSeed")]
    pub loot_table_seed: Option<i64>,
    #[serde(rename = "Lunar")]
    pub lunar: i8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EndGatewayBlockEntityData {
    #[serde(rename = "Age")]
    pub age: i64,
    #[serde(rename = "ExactTeleport")]
    pub extract_teleport: i8,
    #[serde(rename = "ExitPortal")]
    pub exit_portal: Map<String, serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FurnaceBlockEntityData {
    #[serde(rename = "BurnTime")]
    pub burn_time: i16,
    #[serde(rename = "CookTime")]
    pub cook_time: i16,
    #[serde(rename = "CookTimeTotal")]
    pub cook_time_total: i16,
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Lock")]
    pub lock: Option<String>,
    #[serde(rename = "RecipesUsed")]
    pub recipes_used: Map<String, serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HopperBlockEntityData {
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Lock")]
    pub lock: Option<String>,
    #[serde(rename = "LootTable")]
    pub loot_table: Option<String>,
    #[serde(rename = "LootTableSeed")]
    pub loot_table_seed: Option<i64>,
    #[serde(rename = "TransferCooldown")]
    pub transfer_cooldown: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JigSawBlockEntityData {
    pub final_state: String,
    pub joint: Option<String>,
    pub name: String,
    pub pool: String,
    pub target: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JukeBoxBlockEntityData {
    #[serde(rename = "IsPlaying")]
    pub is_playing: i8,
    #[serde(rename = "RecordItem")]
    pub record_item: Map<String, serde_json::Value>,
    #[serde(rename = "RecordStartTick")]
    pub record_start_tick: i64,
    #[serde(rename = "TickCount")]
    pub tick_count: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LecternBlockEntityData {
    #[serde(rename = "Book")]
    pub book: Map<String, serde_json::Value>,
    #[serde(rename = "Page")]
    pub page: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PistonBlockEntityData {
    #[serde(rename = "blockState")]
    pub block_state: Map<String, serde_json::Value>,
    pub extending: i8,
    pub facing: i32,
    pub progress: f32,
    pub source: i8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShulkerBoxBlockEntityData {
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Lock")]
    pub lock: Option<String>,
    #[serde(rename = "LootTable")]
    pub loot_table: Option<String>,
    #[serde(rename = "LootTableSeed")]
    pub loot_table_seed: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkullBlockEntityData {
    pub note_block_sound: Option<String>,
    pub extra_type: Option<String>,
    pub skull_owner: Map<String, serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmokerBlockEntityData {
    #[serde(rename = "BurnTime")]
    pub burn_time: i16,
    #[serde(rename = "CookTime")]
    pub cook_time: i16,
    #[serde(rename = "CookTimeTotal")]
    pub cook_time_total: i16,
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Lock")]
    pub lock: Option<String>,
    #[serde(rename = "RecipesUsed")]
    pub recipes_used: Map<String, serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoulCampFireBlockEntityData {
    #[serde(rename = "CookingTimes")]
    pub cooking_times: Vec<i32>,
    #[serde(rename = "CookingTotalTimes")]
    pub cooking_total_times: Vec<i32>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrappedChestBlockEntityData {
    #[serde(rename = "CustomName")]
    pub custom_name: Option<String>,
    #[serde(rename = "Items")]
    pub items: Vec<Item>,
    #[serde(rename = "Lock")]
    pub lock: Option<String>,
    #[serde(rename = "LootTable")]
    pub loot_table: Option<String>,
    #[serde(rename = "LootTableSeed")]
    pub loot_table_seed: Option<i64>,
    pub gold: Option<i8>,
}
