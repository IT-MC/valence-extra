use std::io::Read;

use crate::{
    BlockEntityData, BlockEntitySchematic, DefaultBlockEntityData, Schematic, SchematicError,
    SignBlockEntityData, SignTextData,
};
use valence_nbt::{Compound, List, Value};

impl Schematic {
    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self, SchematicError> {
        let (nbt, _) = if &bytes.len() >= &2 && &bytes[0..2] == [0x1F, 0x8B] {
            let mut new_bytes = flate2::read::GzDecoder::new(bytes);
            let mut buffer = Vec::new();
            let _ = new_bytes
                .read_to_end(&mut buffer)
                .map_err(|_| SchematicError::ParseError(String::new()));

            valence_nbt::from_binary::<String>(&mut buffer.as_slice())
                .map_err(|_| SchematicError::ParseError(String::new()))?
        } else {
            valence_nbt::from_binary::<String>(&mut bytes)
                .map_err(|_| SchematicError::ParseError(String::new()))?
        };

        let schematic_value = nbt.get("Schematic");
        let schematic_compound = match schematic_value {
            Some(Value::Compound(compound)) => compound,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        // Size along X, Y and Z axis
        let width = match schematic_compound.get("Width") {
            Some(Value::Short(width)) => width,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        let height = match schematic_compound.get("Height") {
            Some(Value::Short(height)) => height,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        let length = match schematic_compound.get("Length") {
            Some(Value::Short(length)) => length,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        // Blocks Wrapper
        let blocks = match schematic_compound.get("Blocks") {
            Some(Value::Compound(blocks)) => blocks,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };
        // Block Values
        let block_entities = match blocks.get("BlockEntities") {
            Some(Value::List(block_entities)) => block_entities,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        let vec_block_entities = Self::parse_vec_block(block_entities.clone()).unwrap();

        let data = match blocks.get("Data") {
            Some(Value::ByteArray(data)) => data,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };
        let palette = match blocks.get("Palette") {
            Some(Value::Compound(palette)) => palette,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        let palette_map = serde_json::to_string(&palette)
            .map_err(|_| SchematicError::ParseError("Failed to serialize compound".to_string()));

        // Entities
        let entities = match schematic_compound.get("Entities") {
            Some(Value::List(entity)) => entity,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        let vec_entities = Self::parse_vec_compound(entities.clone()).unwrap();

        Ok(Schematic {
            w: *width,
            h: *height,
            l: *length,
            block_entities: vec_block_entities.clone(),
            data: data.clone(),
            palette: palette_map?,
            entities: vec_entities,
        })
    }

    fn parse_vec_compound(list: List<String>) -> Result<Vec<String>, SchematicError> {
        list.into_iter()
            .map(|x| match x {
                Value::Compound(compound) => serde_json::to_string(&compound).map_err(|_| {
                    SchematicError::ParseError("Failed to serialize compound".to_string())
                }),
                _ => Err(SchematicError::InvalidFormat(
                    "Expected Compound value".to_string(),
                )),
            })
            .collect()
    }

    fn parse_compound_block(compound: Compound) -> Result<BlockEntitySchematic, SchematicError> {
        // Data is optional according to Sponge Schematic Specification
        let data = match compound.get("Data") {
            Some(Value::Compound(data)) => data,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        let id = match compound.get("Id") {
            Some(Value::String(data)) => data,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        let block_entity_data = BlockEntityData::parse_block_entity_data(data.clone());

        let pos = match compound.get("Pos") {
            Some(Value::IntArray(data)) => data,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        Ok(BlockEntitySchematic {
            data: block_entity_data,
            id: id.to_string(),
            pos: pos.clone(),
        })
    }

    fn parse_vec_block(list: List<String>) -> Result<Vec<BlockEntitySchematic>, SchematicError> {
        list.into_iter()
            .map(|x| match x {
                Value::Compound(compound) => Ok(Self::parse_compound_block(compound)?),
                _ => Err(SchematicError::InvalidFormat(
                    "Expected Compound value".to_string(),
                )),
            })
            .collect()
    }
}

impl BlockEntityData {
    fn parse_block_entity_data(compound: Compound) -> BlockEntityData {
        let id = match compound.get("id") {
            Some(Value::String(data)) => data,
            _ => "",
        };

        return match id {
            "minecraft:sign" => {
                let is_waxed = compound
                    .get("is_waxed")
                    .and_then(|val| match val {
                        Value::Byte(v) => Some(*v),
                        _ => None,
                    })
                    .unwrap_or(0);

                // Resolve back_text and front_text
                let front_text = compound
                    .get("front_text")
                    .and_then(|val| match val {
                        Value::Compound(v) => Some(v.clone()),
                        _ => None,
                    })
                    .unwrap_or(Compound::new());

                let front_text_color = front_text
                    .get("color")
                    .and_then(|val| match val {
                        Value::String(v) => Some(v.clone()),
                        _ => None,
                    })
                    .unwrap_or("yellow".to_string());

                let front_text_has_glowing_text = front_text
                    .get("has_glowing_text")
                    .and_then(|val| match val {
                        Value::Byte(v) => Some(*v),
                        _ => None,
                    })
                    .unwrap_or(0);

                let front_text_messages: Vec<String> = match front_text.get("messages") {
                    Some(Value::List(data)) => data
                        .clone()
                        .into_iter()
                        .map(|x| match x {
                            Value::String(text) => text,
                            _ => "".to_string(),
                        })
                        .collect(),
                    _ => vec![],
                };

                // Back Text Values
                let back_text = compound
                    .get("back_text")
                    .and_then(|val| match val {
                        Value::Compound(v) => Some(v.clone()),
                        _ => None,
                    })
                    .unwrap_or(Compound::new());

                let back_text_color = back_text
                    .get("color")
                    .and_then(|val| match val {
                        Value::String(v) => Some(v.clone()),
                        _ => None,
                    })
                    .unwrap_or("black".to_string());

                let back_text_has_glowing_text = back_text
                    .get("has_glowing_text")
                    .and_then(|val| match val {
                        Value::Byte(v) => Some(*v),
                        _ => None,
                    })
                    .unwrap_or(0);

                let back_text_messages: Vec<String> = match back_text.get("messages") {
                    Some(Value::List(data)) => data
                        .clone()
                        .into_iter()
                        .map(|x| match x {
                            Value::String(text) => text,
                            _ => "".to_string(),
                        })
                        .collect(),
                    _ => vec![],
                };

                BlockEntityData::Sign(SignBlockEntityData {
                    id: id.to_string(),
                    is_waxed,
                    front_text: SignTextData {
                        color: front_text_color,
                        has_glowing_text: front_text_has_glowing_text,
                        messages: front_text_messages,
                    },
                    back_text: SignTextData {
                        color: back_text_color,
                        has_glowing_text: back_text_has_glowing_text,
                        messages: back_text_messages,
                    },
                })
            }
            _ => BlockEntityData::Default(DefaultBlockEntityData { id: id.to_string() }),
        };
    }
}