pub mod decode;

use std::path::Path;
use thiserror::Error;

pub struct Schematic {
    w: f32,
    h: f32,
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
