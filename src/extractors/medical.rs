use crate::interfaces::EntityExtractor;
use crate::schemas::{Entity, EntityType};
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct MedicalExtractor {
    patterns: Vec<Regex>,
}

impl MedicalExtractor {
    pub fn new() -> Self {
        let mut extractor = Self {
            patterns: Vec::new(),
        };
        extractor.compile_patterns();
        extractor
    }
}

impl Default for MedicalExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl MedicalExtractor {
    fn compile_patterns(&mut self) {
        self.patterns = vec![
            Regex::new(
                r"(?i)\b(?:Hospital|Medical Center|Clinic)[:\s]+([A-Z][a-zA-Z\s]+?)(?:\s+[A-Z]|$|[,.]|\n)"
            ).unwrap(),
            Regex::new(
                r"(?i)\b([A-Z][a-z]+(?:\s+[A-Z][a-z]+){0,2}\s+(?:Hospital|Medical Center|Clinic))\b"
            ).unwrap(),
        ];
    }
}

impl EntityExtractor for MedicalExtractor {
    fn name(&self) -> &str {
        "MedicalExtractor"
    }

    fn supported_types(&self) -> Vec<EntityType> {
        vec![EntityType::Medical]
    }

    fn extract(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();
        let mut detected = HashSet::new();
        
        for pattern in &self.patterns {
            for caps in pattern.captures_iter(text) {
                let value = if let Some(matched) = caps.get(1) {
                    matched.as_str().trim().to_string()
                } else if let Some(matched) = caps.get(0) {
                    matched.as_str().trim().to_string()
                } else {
                    continue;
                };
                
                if detected.contains(&value) || value.len() < 5 {
                    continue;
                }
                
                let upper = value.to_uppercase();
                if upper == "HOSPITAL" || upper == "MEDICAL CENTER" || upper == "CLINIC" {
                    continue;
                }
                
                let word_count = value.split_whitespace().count();
                if word_count > 5 {
                    continue;
                }
                
                let (start, end) = if let Some(matched) = caps.get(1) {
                    (matched.start(), matched.end())
                } else if let Some(matched) = caps.get(0) {
                    (matched.start(), matched.end())
                } else {
                    continue;
                };
                
                detected.insert(value.clone());
                entities.push(Entity {
                    entity_type: EntityType::Medical,
                    value,
                    start,
                    end,
                    confidence: 0.80,
                    placeholder: None,
                    metadata: HashMap::new(),
                });
            }
        }
        
        entities
    }
}