use crate::extractors::base::BaseExtractor;
use crate::schemas::{Entity, EntityType};
use regex::Regex;

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

impl BaseExtractor for DateExtractor {
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
    
    fn supported_types(&self) -> Vec<EntityType> {
        vec![EntityType::Date]
    }
    
    fn extract(&self, text: &str) -> Vec<Entity> {
        if let Err(e) = self.validate_text(text) {
            log::warn!("Validation failed: {}", e);
            return Vec::new();
        }
        
        self.extract_matches(text, &self.patterns, EntityType::Date, 0.90)
    }
}
