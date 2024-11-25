pub mod block_entity_data;
pub mod decode;

use block_entity_data::BlockEntityData;
use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Map;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct SchematicWrapper {
    #[serde(rename = "Schematic")]
    pub schematic: Schematic,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlocksWrapper {
    #[serde(rename = "BlockEntities")]
    pub block_entities: Vec<BlockEntitySchematic>,
    #[serde(rename = "Data", deserialize_with = "deserialize_byte_array")]
    pub data: Vec<i8>,
    #[serde(rename = "Palette")]
    pub palette: Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schematic {
    #[serde(rename = "Width")]
    pub w: i16,
    #[serde(rename = "Height")]
    pub h: i16,
    #[serde(rename = "Length")]
    pub l: i16,
    #[serde(rename = "Blocks")]
    pub blocks: BlocksWrapper,
    #[serde(rename = "Entities")]
    pub entities: Vec<Map<String, serde_json::Value>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockEntitySchematic {
    #[serde(rename = "Data")]
    pub data: BlockEntityData,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Pos")]
    pub pos: Vec<i16>,
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

fn deserialize_byte_array<'de, D>(deserializer: D) -> Result<Vec<i8>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ByteArrayVisitor;

    impl<'de> Visitor<'de> for ByteArrayVisitor {
        type Value = Vec<i8>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte array (Vec<i8>)")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = seq.next_element()? {
                values.push(value);
            }
            Ok(values)
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v.iter().map(|&byte| byte as i8).collect())
        }
    }

    deserializer.deserialize_any(ByteArrayVisitor)
}
