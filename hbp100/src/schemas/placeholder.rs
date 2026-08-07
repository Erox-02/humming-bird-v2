use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Placeholder {
    pub placeholder: String,
    pub original_value: String,
    pub entity_type: String,
}

impl Placeholder {
    pub fn new(placeholder: impl Into<String>, original_value: impl Into<String>, entity_type: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            original_value: original_value.into(),
            entity_type: entity_type.into(),
        }
    }

    pub fn to_metadata(&self) -> (String, String) {
        (self.placeholder.clone(), self.original_value.clone())
    }
}
