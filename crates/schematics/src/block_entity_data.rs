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
    Conduit(DefaultBlockEntityData),
    #[serde(rename = "minecraft:daylight_detector")]
    DayLightDetector(DefaultBlockEntityData),
    #[serde(rename = "minecraft:decorated_pot")]
    DecoratedPot(DefaultBlockEntityData),
    #[serde(rename = "minecraft:dispenser")]
    Dispenser(DefaultBlockEntityData),
    #[serde(rename = "minecraft:dropper")]
    Dropper(DefaultBlockEntityData),
    #[serde(rename = "minecraft:enchanting_table")]
    EnchantingTable(DefaultBlockEntityData),
    #[serde(rename = "minecraft:ender_chest")]
    EnderChest(DefaultBlockEntityData),
    #[serde(rename = "minecraft:end_gateway")]
    EndGateway(DefaultBlockEntityData),
    #[serde(rename = "minecraft:end_portal")]
    EndPortal(DefaultBlockEntityData),
    #[serde(rename = "minecraft:furnance")]
    Furnance(DefaultBlockEntityData),
    #[serde(rename = "minecraft:hopper")]
    Hopper(DefaultBlockEntityData),
    #[serde(rename = "minecraft:jigsaw")]
    JigSaw(DefaultBlockEntityData),
    #[serde(rename = "minecraft:jukebox")]
    JukeBox(DefaultBlockEntityData),
    #[serde(rename = "minecraft:lectern")]
    Lectern(DefaultBlockEntityData),
    #[serde(rename = "minecraft:mob_spawner")]
    MobSpawner(DefaultBlockEntityData),
    #[serde(rename = "minecraft:piston")]
    Piston(DefaultBlockEntityData),
    #[serde(rename = "minecraft:shulker_box")]
    ShulkerBox(DefaultBlockEntityData),
    #[serde(rename = "minecraft:skull")]
    Skull(DefaultBlockEntityData),
    #[serde(rename = "minecraft:smoker")]
    Smoker(DefaultBlockEntityData),
    #[serde(rename = "minecraft:soul_campfire")]
    SoulCampFire(DefaultBlockEntityData),
    #[serde(rename = "minecraft:structure_block")]
    StructureBlock(DefaultBlockEntityData),
    #[serde(rename = "minecraft:trapped_chest")]
    TrappedChest(DefaultBlockEntityData),
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
pub struct CommandBlockEntityData;
