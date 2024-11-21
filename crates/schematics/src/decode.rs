use std::{io::Read, path::Path};

use crate::{BlockEntitySchematic, BlockEntityData, Schematic, SchematicError};
use valence::prelude::Res;
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

        let parsed_data = serde_json::to_string(&data).expect("Failed to serialize compound");

        let id = match compound.get("Id") {
            Some(Value::String(data)) => data,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        let pos = match compound.get("Pos") {
            Some(Value::IntArray(data)) => data,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        Ok(BlockEntitySchematic {
            data: Some(BlockEntityData::Sign(parsed_data)),
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

    /*     fn list_to_vec<S>(list: List<S>) -> Vec<Box<dyn std::any::Any>>
    where
        S: ToString + 'static + serde::ser::Serialize + std::hash::Hash + std::cmp::Ord,
    {
        match list {
            List::End => Vec::new(),
            List::Byte(list) => list
                .into_iter()
                .map(|x| Box::new(x) as Box<dyn std::any::Any>)
                .collect(),
            List::Short(list) => list
                .into_iter()
                .map(|x| Box::new(x) as Box<dyn std::any::Any>)
                .collect(),
            List::Int(list) => list
                .into_iter()
                .map(|x| Box::new(x) as Box<dyn std::any::Any>)
                .collect(),
            List::Long(list) => list
                .into_iter()
                .map(|x| Box::new(x) as Box<dyn std::any::Any>)
                .collect(),
            List::Float(list) => list
                .into_iter()
                .map(|x| Box::new(x) as Box<dyn std::any::Any>)
                .collect(),
            List::Double(list) => list
                .into_iter()
                .map(|x| Box::new(x) as Box<dyn std::any::Any>)
                .collect(),
            List::String(list) => list
                .into_iter()
                .map(|x| Box::new(x) as Box<dyn std::any::Any>)
                .collect(),
            List::ByteArray(list) => list
                .into_iter()
                .flat_map(|inner| {
                    inner
                        .into_iter()
                        .map(|x| Box::new(x) as Box<dyn std::any::Any>)
                })
                .collect(),
            List::IntArray(list) => list
                .into_iter()
                .flat_map(|inner| {
                    inner
                        .into_iter()
                        .map(|x| Box::new(x) as Box<dyn std::any::Any>)
                })
                .collect(),
            List::LongArray(list) => list
                .into_iter()
                .flat_map(|inner| {
                    inner
                        .into_iter()
                        .map(|x| Box::new(x) as Box<dyn std::any::Any>)
                })
                .collect(),
            List::List(list) => list.into_iter().flat_map(Self::list_to_vec).collect(),
            List::Compound(list) => list
                .into_iter()
                .map(|x| {
                    let json_string = serde_json::to_string(&x).expect("Serialization failed");
                    Box::new(json_string) as Box<dyn std::any::Any>
                })
                .collect(),
        }
    } */
}
