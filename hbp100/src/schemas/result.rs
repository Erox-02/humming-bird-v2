use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{Entity, PrivacyDecision};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessResult {
    pub original_text: String,
    pub masked_text: String,
    pub metadata: HashMap<String, String>,
    pub entities: Vec<Entity>,
    pub decisions: Vec<PrivacyDecision>,
    pub has_pii: bool,
}

impl ProcessResult {
    pub fn new(
        original_text: impl Into<String>,
        masked_text: impl Into<String>,
    ) -> Self {
        Self {
            original_text: original_text.into(),
            masked_text: masked_text.into(),
            metadata: HashMap::new(),
            entities: Vec::new(),
            decisions: Vec::new(),
            has_pii: false,
        }
    }

    pub fn to_dict(&self) -> serde_json::Value {
        serde_json::json!({
            "original_text": self.original_text,
            "masked_text": self.masked_text,
            "metadata": self.metadata,
            "has_pii": self.has_pii,
            "entities": self.entities.iter().map(|e| e.to_dict()).collect::<Vec<_>>(),
            "decisions": self.decisions.iter().map(|d| d.to_dict()).collect::<Vec<_>>(),
        })
    }
}
