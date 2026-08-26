use serde::{Deserialize, Serialize};
use regex::Regex;
use std::collections::HashMap;
use crate::schemas::{Entity, EntityType};
use crate::base::Extractor;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtractorConfig {
    pub name: String,
    pub entity_type: String,
    pub pattern: String,
    pub priority: Option<u8>,
    pub flags: Option<Vec<String>>,
    pub context_rules: Option<Vec<ContextRule>>,
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRule {
    pub before: Option<String>,
    pub after: Option<String>,
    pub required: bool,
}
pub struct ConfigurableExtractor {
    config: ExtractorConfig,
    regex: Regex,
}

impl ConfigurableExtractor {
    pub fn new(config: ExtractorConfig) -> Result<Self, String> {
        let regex = Regex::new(&config.pattern)
            .map_err(|e| format!("Invalid regex: {}", e))?;
        Ok(Self { config, regex })
    }
}

impl Extractor for ConfigurableExtractor {
    fn name(&self) -> &str {
        &self.config.name
    }   
    fn supported_types(&self) -> Vec<EntityType> {
        vec![EntityType::from(self.config.entity_type.clone())]
    }
    fn extract(&self, text: &str) -> Vec<Entity> {
        let confidence = self.config.confidence.unwrap_or(0.85);
        self.regex
            .find_iter(text)
            .map(|m| Entity {
                entity_type: EntityType::from(self.config.entity_type.clone()),
                value: m.as_str().to_string(),
                start: m.start(),
                end: m.end(),
                confidence,
            })
            .collect()
    }
}