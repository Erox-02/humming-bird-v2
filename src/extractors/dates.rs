use crate::interfaces::EntityExtractor;
use crate::schemas::{Entity, EntityType};
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct DateExtractor {
    patterns: Vec<Regex>,
}

impl DateExtractor {
    pub fn new() -> Self {
        let mut extractor = Self {
            patterns: Vec::new(),
        };
        extractor.compile_patterns();
        extractor
    }
}

impl Default for DateExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl DateExtractor {
    fn compile_patterns(&mut self) {
        self.patterns = vec![
            Regex::new(r"\b\d{1,2}[/-]\d{1,2}[/-]\d{2,4}\b").unwrap(),
            Regex::new(r"\b\d{4}[/-]\d{1,2}[/-]\d{1,2}\b").unwrap(),
            Regex::new(r"\b[A-Z][a-z]+ \d{1,2},? \d{4}\b").unwrap(),
            Regex::new(r"\b\d{1,2} [A-Z][a-z]+ \d{4}\b").unwrap(),
            Regex::new(r"\b[A-Z][a-z]+ \d{4}\b").unwrap(),
            Regex::new(r"\b\d{1,2}[/-]\d{1,2}\b").unwrap(),
        ];
    }
}

impl EntityExtractor for DateExtractor {
    fn name(&self) -> &str {
        "DateExtractor"
    }

    fn supported_types(&self) -> Vec<EntityType> {
        vec![EntityType::Date]
    }

    fn extract(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();
        let mut detected = HashSet::new();
        
        for pattern in &self.patterns {
            for m in pattern.find_iter(text) {
                let value = m.as_str().to_string();
                if !detected.contains(&value) {
                    detected.insert(value.clone());
                    entities.push(Entity {
                        entity_type: EntityType::Date,
                        value,
                        start: m.start(),
                        end: m.end(),
                        confidence: 0.90,
                        placeholder: None,
                        metadata: HashMap::new(),
                    });
                }
            }
        }
        
        entities
    }
}