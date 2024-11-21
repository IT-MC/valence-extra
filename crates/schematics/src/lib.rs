pub mod decode;

use thiserror::Error;
use valence_nbt::Value;
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
    pub data: Option<BlockEntityData>,
    pub id: String,
    pub pos: Vec<i32>,
}

#[derive(Clone, Debug)]
pub enum BlockEntityData {
    Sign(String),
    Default(String)
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
