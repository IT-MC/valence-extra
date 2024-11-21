pub mod decode;

use thiserror::Error;
use valence_nbt::Value;
pub struct Schematic {
    w: i16,
    h: i16,
    l: i16,
    palette: Value,
    pub block_entities: Vec<Box<dyn std::any::Any>>,
    data: Vec<i8>,
    pub entities: Vec<Box<dyn std::any::Any>>,
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
