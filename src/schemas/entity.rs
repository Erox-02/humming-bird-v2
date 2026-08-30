use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EntityType {
    Name,
    Email,
    Phone,
    Date,
    Address,
    Id,
    Medical,
    Custom(String),
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
            _ => EntityType::Custom(s),
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
            EntityType::Custom(s) => s,
        }
    }
}