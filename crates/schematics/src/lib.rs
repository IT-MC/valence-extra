pub mod decode;

use thiserror::Error;
pub struct Schematic {
    pub w: i16,
    pub h: i16,
    pub l: i16,
    pub palette: String,
    pub block_entities: Vec<BlockEntitySchematic>,
    pub data: Vec<i8>,
    pub entities: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct BlockEntitySchematic {
    pub data: BlockEntityData,
    pub id: String,
    pub pos: Vec<i32>,
}

#[derive(Debug, Error)]
pub enum SchematicError {
    #[error("invalid format `{0}`")]
    InvalidFormat(String),
    #[error("file not found!")]
    FileNotFound(String),
    #[error("parse error `{0}`")]
    ParseError(String),
    #[error("unsupported version `{0}`")]
    UnsupportedVersion(u32),
}

#[derive(Clone, Debug)]
pub enum BlockEntityData {
    Sign(SignBlockEntityData),
    Barrel(String),
    Beacon(String),
    Bed(String),
    Beehive(String),
    Bell(String),
    BlastFurnance(String),
    BrewingStand(String),
    CampFire(String),
    ChiseledBookShelf(String),
    Chest(String),
    Comparator(String),
    CommandBlock(String),
    Conduit(String),
    DayLightDetector(String),
    Dispenser(String),
    Dropper(String),
    EnchantingTable(String),
    EnderChest(String),
    EndGateway(String),
    EndPortal(String),
    Furnance(String),
    Hopper(String),
    JigSaw(String),
    JukeBox(String),
    Lectern(String),
    MobSpawner(String),
    Piston(String),
    ShulkerBox(String),
    Skull(String),
    Smoker(String),
    SoulCampFire(String),
    StructureBLock(String),
    TrappedChest(String),
    Default(DefaultBlockEntityData),
}

#[derive(Clone, Debug)]
pub struct DefaultBlockEntityData {
    pub id: String,
}

#[derive(Clone, Debug)]
pub struct SignBlockEntityData {
    back_text: SignTextData,
    front_text: SignTextData,
    pub id: String,
    is_waxed: i8,
}

#[derive(Clone, Debug)]
struct SignTextData {
    color: String,
    has_glowing_text: i8,
    messages: Vec<String>,
}
