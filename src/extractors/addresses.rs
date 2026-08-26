use crate::interfaces::EntityExtractor;
use crate::schemas::{Entity, EntityType};
use regex::Regex;
use std::collections::HashSet;

pub struct AddressExtractor {
    patterns: Vec<Regex>,
}

impl AddressExtractor {
    pub fn new() -> Self {
        let mut extractor = Self {
            patterns: Vec::new(),
        };
        extractor.compile_patterns();
        extractor
    }
}

impl Default for AddressExtractor {
    fn default() -> Self {
        Self::new()
    }
}
impl AddressExtractor {
    fn compile_patterns(&mut self) {
        self.patterns = vec![
            Regex::new(
                r"(?i)\b(?:Address|Mailing Address|Home Address)[:\s]+([^.\n]{10,100}?)(?:\s+(?:and|phone|email|policy|ssn|mrn|passport|[A-Z]{2,}\d)|\.|\n|$)"
            ).unwrap(),
            Regex::new(
                r"(?i)\b(\d{1,5}\s+[A-Za-z]+\s+(?:Street|St|Avenue|Ave|Road|Rd|Boulevard|Blvd|Lane|Ln|Drive|Dr|Way|Place|Pl|Court|Ct)[,\s]+[A-Za-z]+[\s,]+[A-Z]{2}\s+\d{5}(?:-\d{4})?)(?:\s+(?:and|phone|email|policy|ssn|mrn|passport|[A-Z]{2,}\d)|\n|\.\s|\.$)"
            ).unwrap(),
            Regex::new(
                r"(?i)\b(\d{1,5}\s+[A-Za-z]+\s+(?:Street|St|Avenue|Ave|Road|Rd|Boulevard|Blvd|Lane|Ln|Drive|Dr|Way|Place|Pl|Court|Ct)[,\s]+[A-Za-z]+[\s,]+[A-Z]{2}\s+\d{5}(?:-\d{4})?)\b"
            ).unwrap(),
        ];
    }
}

impl EntityExtractor for AddressExtractor {
    fn name(&self) -> &str {
        "AddressExtractor"
    }

    fn supported_types(&self) -> Vec<EntityType> {
        vec![EntityType::ADDRESS]
    }

    fn extract(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();
        let mut detected = HashSet::new();
        
        for pattern in &self.patterns {
            for caps in pattern.captures_iter(text) {
                if let Some(matched) = caps.get(1) {
                    let mut value = matched.as_str().trim().to_string();
                    value = value.trim_end_matches(|c| c == '.' || c == ',').to_string();
                    
                    if !detected.contains(&value) && value.len() >= 10 {
                        detected.insert(value.clone());
                        entities.push(Entity {
                            entity_type: EntityType::ADDRESS,
                            value,
                            start: matched.start(),
                            end: matched.end(),
                            confidence: 0.80,
                        });
                    }
                }
            }
        }       
        entities
    }
}