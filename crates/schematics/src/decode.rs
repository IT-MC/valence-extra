use std::{io::Read, path::Path};

use crate::{Schematic, SchematicError};

impl Schematic {
    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self, SchematicError> {
        let (nbt, root_name) = if &bytes.len() >= &2 && &bytes[0..2] == [0x1F, 0x8B] {
            let mut new_bytes = flate2::read::GzDecoder::new(bytes);
            let mut buffer = Vec::new();
            let _ = new_bytes
                .read_to_end(&mut buffer)
                //.unwrap();
                .map_err(|_| SchematicError::ParseError(String::new()));

            valence_nbt::from_binary::<String>(&mut buffer.as_slice())
                //.unwrap()
                .map_err(|_| SchematicError::ParseError(String::new()))?
        } else {
            valence_nbt::from_binary::<String>(&mut bytes)
                .map_err(|_| SchematicError::ParseError(String::new()))?
        };
        println!("NBT: {:?}", nbt);
        println!("Root Name: {:?}", root_name);
        Ok(Schematic { w: 0.0, h: 0.0 })
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, SchematicError> {
        Self::from_bytes(
            std::fs::read(path)
                .map_err(|_| SchematicError::FileNotFound(String::new()))?
                .as_slice(),
        )
    }
}
