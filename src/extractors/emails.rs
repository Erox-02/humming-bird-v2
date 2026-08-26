use crate::interfaces::EntityExtractor;
use crate::schemas::{Entity, EntityType};
use regex::Regex;
use std::collections::HashSet;

pub struct EmailExtractor {
    patterns: Vec<Regex>,
}

impl EmailExtractor {
    pub fn new() -> Self {
        let mut extractor = Self {
            patterns: Vec::new(),
        };
        extractor.compile_patterns();
        extractor
    }
}

impl Default for EmailExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl EmailExtractor {
    fn compile_patterns(&mut self) {
        self.patterns = vec![
            Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b").unwrap(),
        ];
    }
}

impl EntityExtractor for EmailExtractor {
    fn name(&self) -> &str {
        "EmailExtractor"
    }

    fn supported_types(&self) -> Vec<EntityType> {
        vec![EntityType::EMAIL]
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
                        entity_type: EntityType::EMAIL,
                        value,
                        start: m.start(),
                        end: m.end(),
                        confidence: 0.99,
                    });
                }
            }
        }
        
        entities
    }
}