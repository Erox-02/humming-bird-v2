use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Entity {
    pub entity_type: EntityType,
    pub value: String,
    pub start: usize,
    pub end: usize,
    pub confidence: f32,
    pub placeholder: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl Entity {
    pub fn new(
        entity_type: EntityType,
        value: String,
        start: usize,
        end: usize,
        confidence: f32,
    ) -> Self {
        Self {
            entity_type,
            value,
            start,
            end,
            confidence,
            placeholder: None,
            metadata: HashMap::new(),
        }
    }

    pub fn to_dict(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("type".to_string(), self.entity_type.as_str().to_string());
        map.insert("value".to_string(), self.value.clone());
        map.insert("start".to_string(), self.start.to_string());
        map.insert("end".to_string(), self.end.to_string());
        map.insert("confidence".to_string(), self.confidence.to_string());
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum EntityType {
    Name,
    Email,
    Phone,
    Date,
    Address,
    Id,
    Medical,
}

impl From<String> for EntityType {
    fn from(s: String) -> Self {
        match s.to_uppercase().as_str() {
            "NAME" => EntityType::Name,
            "EMAIL" => EntityType::Email,
            "PHONE" => EntityType::Phone,
            "DATE" => EntityType::Date,
            "ADDRESS" => EntityType::Address,
            "ID" => EntityType::Id,
            "MEDICAL" => EntityType::Medical,
            _ => EntityType::Id,
        }
    }
}

impl EntityType {
    pub fn as_str(&self) -> &str {
        match self {
            EntityType::Name => "NAME",
            EntityType::Email => "EMAIL",
            EntityType::Phone => "PHONE",
            EntityType::Date => "DATE",
            EntityType::Address => "ADDRESS",
            EntityType::Id => "ID",
            EntityType::Medical => "MEDICAL",
        }
    }
}