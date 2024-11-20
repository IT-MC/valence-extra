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
            block_entities: Value::List(block_entities.clone()),
            data: data.clone(),
            palette: Value::Compound(palette.clone()),
            entities: Value::List(entities.clone()),
        })
    }
}
