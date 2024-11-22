use std::io::Read;

use crate::{BlockEntityData, BlockEntitySchematic, Schematic, SchematicError};
use valence_nbt::{Compound, List, Value};

impl Schematic {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SchematicError> {
        let schematic_compound = Self::decode_nbt_data(bytes)?;

        // Size along W, H and L axis
        let (width, height, length) = Self::extract_dimensions(&schematic_compound)?;

        // Blocks Wrapper
        let blocks = match schematic_compound.get("Blocks") {
            Some(Value::Compound(blocks)) => blocks,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        // BlockEntities
        let block_entities = match blocks.get("BlockEntities") {
            Some(Value::List(block_entities)) => block_entities,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        let vec_block_entities =
            Self::parse_vec_generic(block_entities.clone(), Self::parse_to_block_entity)?;

        let data = match blocks.get("Data") {
            Some(Value::ByteArray(data)) => data,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

        // Palette
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

        let vec_entities =
            Self::parse_vec_generic(entities.clone(), |compound: valence_nbt::Compound| {
                serde_json::to_string(&compound)
                    .map_err(|e| SchematicError::ParseError(format!("Serialization error: {}", e)))
            })?;

        Ok(Schematic {
            w: width,
            h: height,
            l: length,
            block_entities: vec_block_entities,
            data: data.to_vec(),
            palette: palette_map?,
            entities: vec_entities,
        })
    }

    fn decode_nbt_data(bytes: &[u8]) -> Result<Compound, SchematicError> {
        let decoded = if bytes.len() >= 2 && bytes[0..2] == [0x1F, 0x8B] {
            let mut decoder = flate2::read::GzDecoder::new(bytes);
            let mut buffer = Vec::new();
            decoder
                .read_to_end(&mut buffer)
                .map_err(|e| SchematicError::ParseError(format!("Gzip decode error: {}", e)))?;
            buffer
        } else {
            bytes.to_vec()
        };

        let (nbt, _) = valence_nbt::from_binary::<String>(&mut decoded.as_slice())
            .map_err(|e| SchematicError::ParseError(format!("NBT parse error: {}", e)))?;

        match nbt.get("Schematic") {
            Some(Value::Compound(compound)) => Ok(compound.clone()),
            _ => Err(SchematicError::InvalidFormat(
                "Missing Schematic compound".to_string(),
            )),
        }
    }

    fn extract_dimensions(compound: &Compound) -> Result<(i16, i16, i16), SchematicError> {
        Ok((
            compound
                .get("Width")
                .and_then(|v| v.as_i16())
                .ok_or_else(|| SchematicError::InvalidFormat("Missing Width".to_string()))?,
            compound
                .get("Height")
                .and_then(|v| v.as_i16())
                .ok_or_else(|| SchematicError::InvalidFormat("Missing Height".to_string()))?,
            compound
                .get("Length")
                .and_then(|v| v.as_i16())
                .ok_or_else(|| SchematicError::InvalidFormat("Missing Length".to_string()))?,
        ))
    }

    fn parse_to_block_entity(compound: Compound) -> Result<BlockEntitySchematic, SchematicError> {
        // Data is optional according to Sponge Schematic Specification
        let data = match compound.get("Data") {
            Some(Value::Compound(data)) => data,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };
        let block_entity_data = BlockEntityData::parse_block_entity_data(data);

        let id = match compound.get("Id") {
            Some(Value::String(data)) => data,
            _ => return Err(SchematicError::InvalidFormat(String::new())),
        };

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

    fn parse_vec_generic<T>(
        list: List<String>,
        parse_fn: impl Fn(Compound) -> Result<T, SchematicError>,
    ) -> Result<Vec<T>, SchematicError> {
        list.into_iter()
            .map(|x| match x {
                Value::Compound(compound) => parse_fn(compound),
                _ => Err(SchematicError::InvalidFormat(
                    "Expected Compound value".to_string(),
                )),
            })
            .collect()
    }
}
