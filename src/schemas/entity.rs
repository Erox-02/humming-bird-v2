use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    Name,
    Physician,
    PatientId,
    Email,
    Phone,
    Address,
    MRN,
    CaseId,
    PolicyNumber,
    Date,
    DOB,
    SSN,
    Hospital,
    Passport,
}

impl EntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityType::Name => "NAME",
            EntityType::Physician => "PHYSICIAN",
            EntityType::PatientId => "PATIENT_ID",
            EntityType::Email => "EMAIL",
            EntityType::Phone => "PHONE",
            EntityType::Address => "ADDRESS",
            EntityType::MRN => "MRN",
            EntityType::CaseId => "CASE_ID",
            EntityType::PolicyNumber => "POLICY_NUMBER",
            EntityType::Date => "DATE",
            EntityType::DOB => "DOB",
            EntityType::SSN => "SSN",
            EntityType::Hospital => "HOSPITAL",
            EntityType::Passport => "PASSPORT",
        }
    }
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

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
        value: impl Into<String>,
        start: usize,
        end: usize,
        confidence: f32,
    ) -> Self {
        Self {
            entity_type,
            value: value.into(),
            start,
            end,
            confidence,
            placeholder: None,
            metadata: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn text(&self) -> &str {
        &self.value
    }

    pub fn to_dict(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("type".to_string(), serde_json::Value::String(self.entity_type.as_str().to_string()));
        map.insert("value".to_string(), serde_json::Value::String(self.value.clone()));
        map.insert("start".to_string(), serde_json::Value::Number(self.start.into()));
        map.insert("end".to_string(), serde_json::Value::Number(self.end.into()));
        map.insert("confidence".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.confidence as f64).unwrap()));
        if let Some(placeholder) = &self.placeholder {
            map.insert("placeholder".to_string(), serde_json::Value::String(placeholder.clone()));
        }
        map
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.entity_type == other.entity_type && self.value == other.value
    }
}

impl Eq for Entity {}

impl std::hash::Hash for Entity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entity_type.hash(state);
        self.value.hash(state);
    }
}
