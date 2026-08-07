use crate::extractors::base::BaseExtractor;
use crate::interfaces::EntityExtractor;
use crate::schemas::{Entity, EntityType};
use regex::Regex;
use std::collections::HashSet;

pub struct PhoneExtractor {
    patterns: Vec<Regex>,
}

impl PhoneExtractor {
    pub fn new() -> Self {
        let mut extractor = Self {
            patterns: Vec::new(),
        };
        extractor.compile_patterns();
        extractor
    }
}

impl Default for PhoneExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityExtractor for PhoneExtractor {
    fn extract(&self, text: &str) -> Vec<Entity> {
        if let Err(e) = self.validate_text(text) {
            log::warn!("Validation failed: {}", e);
            return Vec::new();
        }
        
        let mut entities = Vec::new();
        let mut detected_digits = HashSet::new();
        
        for pattern in &self.patterns {
            for caps in pattern.captures_iter(text) {
                if let Some(matched) = caps.get(0) {
                    let value = matched.as_str();
                    let cleaned: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
                    
                    if !detected_digits.contains(&cleaned) && (10..=15).contains(&cleaned.len()) {
                        detected_digits.insert(cleaned);
                        entities.push(Entity::new(
                            EntityType::Phone,
                            value,
                            matched.start(),
                            matched.end(),
                            0.85,
                        ));
                    }
                }
            }
        }
        
        entities
    }
    
    fn supported_types(&self) -> Vec<EntityType> {
        vec![EntityType::Phone]
    }
}

impl BaseExtractor for PhoneExtractor {
    fn compile_patterns(&mut self) {
        self.patterns = vec![
            Regex::new(r"\b\+\d{1,3}[-.\s]?\(?\d{1,4}\)?[-.\s]?\d{1,4}[-.\s]?\d{1,9}\b").unwrap(),
            Regex::new(r"(?<!\w)\(?\d{3}\)?[-.\s]?\d{3}[-.\s]?\d{4}\b").unwrap(),
            Regex::new(r"\b\d{3}[-.\s]\d{3}[-.\s]\d{4}\b").unwrap(),
            Regex::new(r"\b\d{10}\b").unwrap(),
        ];
    }
}
