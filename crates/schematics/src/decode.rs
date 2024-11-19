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

impl Schematic {
    pub fn new(mut some_bytes: &[u8]) {
        let (nbt, root_name) = valence_nbt::from_binary::<String>(&mut some_bytes).unwrap();
    }
}
