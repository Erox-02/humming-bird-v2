use serde {Deserialize, Serialize};
use regex::Regex;
use std::collections::HashMap;
use crate::schemas::Entity;
use crate::extractors::Extractor;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtractorConfig {
    pub name: String,
    pub entity_type: String,
    pub pattern: String,
    pub priority: Options<u8>,
    pub flags: Option<Vec<String>>,
    pub context_rules: Option<Vec<ContextRule>,
    pub confidence: Option<f64>,
}

#[debug(Debug, Clone, Serialize, Deserialize)]
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
    fn entity_type(&self) -> &str {
        &self.config.entity_type
    }
    fn extract(&self, text: &str) -> Vec<Entity> {
        self.regex
            .find_iter(text)
            .map(|m| Entity {
                entity_type: self.config.entity_type.clone().into(),
                value: m.as_str().to_string(),
                start: m.start(),
                end: m.end(),
                confidence,
            })
            .collect()
    }
}