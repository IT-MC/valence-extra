use crate::{BlockEntityData, DefaultBlockEntityData, SignBlockEntityData, SignTextData};
use valence_nbt::{Compound, List, Value};

impl BlockEntityData {
    pub fn parse_block_entity_data(compound: &Compound) -> BlockEntityData {
        let id = compound.get("id").and_then(Value::as_str).unwrap_or("");

        match id {
            "minecraft:sign" => {
                let is_waxed = compound
                    .get("is_waxed")
                    .and_then(Value::as_byte)
                    .unwrap_or(0);

                let front_text = Self::parse_sign_text(
                    compound.get("front_text").and_then(Value::as_compound),
                    "yellow".to_string(),
                );

                let back_text = Self::parse_sign_text(
                    compound.get("back_text").and_then(Value::as_compound),
                    "black".to_string(),
                );

                BlockEntityData::Sign(SignBlockEntityData {
                    id: id.to_string(),
                    is_waxed,
                    front_text,
                    back_text,
                })
            }
            _ => BlockEntityData::Default(DefaultBlockEntityData { id: id.to_string() }),
        }
    }

    fn parse_sign_text(text_data: Option<&Compound>, default_color: String) -> SignTextData {
        let default_compound = Compound::new();
        let text = text_data.unwrap_or(&default_compound);
        let color = text
            .get("color")
            .and_then(Value::as_str)
            .map(ToString::to_string)
            .unwrap_or(default_color);

        let has_glowing_text = text
            .get("has_glowing_text")
            .and_then(Value::as_byte)
            .unwrap_or(0);

        let messages = match text.get("messages") {
            Some(Value::List(list)) => list
                .clone()
                .into_iter()
                .map(|x| match x {
                    Value::String(text) => text,
                    _ => String::new(),
                })
                .collect(),
            _ => Vec::new(),
        };

        SignTextData {
            color,
            has_glowing_text,
            messages,
        }
    }
}

trait ValueExt {
    fn as_str(&self) -> Option<&str>;
    fn as_byte(&self) -> Option<i8>;
    fn as_compound(&self) -> Option<&Compound>;
}

impl ValueExt for Value {
    fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    fn as_byte(&self) -> Option<i8> {
        match self {
            Value::Byte(b) => Some(*b),
            _ => None,
        }
    }

    fn as_compound(&self) -> Option<&Compound> {
        match self {
            Value::Compound(c) => Some(c),
            _ => None,
        }
    }
}
