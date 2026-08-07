use crate::extractors::base::BaseExtractor;
use crate::interfaces::EntityExtractor;
use crate::schemas::{Entity, EntityType};
use regex::Regex;

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

impl EntityExtractor for EmailExtractor {
    fn extract(&self, text: &str) -> Vec<Entity> {
        if let Err(e) = self.validate_text(text) {
            log::warn!("Validation failed: {}", e);
            return Vec::new();
        }
        
        self.extract_matches(text, &self.patterns, EntityType::Email, 0.95)
    }
    
    fn supported_types(&self) -> Vec<EntityType> {
        vec![EntityType::Email]
    }
}

impl BaseExtractor for EmailExtractor {
    fn compile_patterns(&mut self) {
        self.patterns = vec![
            Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b").unwrap(),
        ];
    }
}
