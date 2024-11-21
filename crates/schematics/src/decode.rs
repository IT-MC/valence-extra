use std::{io::Read, path::Path};

use crate::{Schematic, SchematicError};
use valence_nbt::{List, Value};

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
        let data = match blocks.get("Data") {
            Some(Value::ByteArray(data)) => data,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };
        let palette = match blocks.get("Palette") {
            Some(Value::Compound(palette)) => palette,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        // Entities
        let entities = match schematic_compound.get("Entities") {
            Some(Value::List(entity)) => entity,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        Ok(Schematic {
            w: *width,
            h: *height,
            l: *length,
            block_entities: Self::list_to_vec(block_entities.clone()),
            data: data.clone(),
            palette: Value::Compound(palette.clone()),
            entities: Self::list_to_vec(entities.clone()),
        })
    }

    fn list_to_vec<S>(list: List<S>) -> Vec<Box<dyn std::any::Any>>
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
    }
}
