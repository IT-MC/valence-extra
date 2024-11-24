use std::io::Read;

use crate::{Schematic, SchematicError, SchematicWrapper};

impl Schematic {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SchematicError> {
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

        let wrapper: SchematicWrapper = serde::Deserialize::deserialize(nbt)
            .map_err(|e| SchematicError::ParseError(format!("Schematic parse error: {}", e)))?;

        Ok(wrapper.schematic)
    }
}
