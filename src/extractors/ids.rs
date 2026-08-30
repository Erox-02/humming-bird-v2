use crate::interfaces::EntityExtractor;
use crate::schemas::{Entity, EntityType};
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct IDConfig {
    labels: Vec<String>,
    min_length: usize,
    pattern: String,
    confidence: f32,
}

pub struct IDExtractor {
    patterns: HashMap<EntityType, Vec<Regex>>,
    configs: HashMap<EntityType, IDConfig>,
}

impl IDExtractor {
    pub fn new() -> Self {
        let mut extractor = Self {
            patterns: HashMap::new(),
            configs: HashMap::new(),
        };
        extractor.compile_patterns();
        extractor
    }
    
    fn get_configs() -> HashMap<EntityType, IDConfig> {
        let mut configs = HashMap::new();
        configs.insert(
            EntityType::Id,
            IDConfig {
                labels: vec![
                    r"\b(?:MRN|Medical Record Number)[:\s]+".to_string(),
                    r"\b(?:Patient ID|PID|Patient Identifier)[:\s]+".to_string(),
                    r"\b(?:Case Number|Case No|Case ID)[:\s]+".to_string(),
                    r"\b(?:Policy Number|Policy No|Policy ID|Insurance Policy)[:\s]+".to_string(),
                    r"\b(?:SSN|Social Security Number)[:\s]+".to_string(),
                    r"\b(?:Passport|Passport Number|Passport No)[:\s]+".to_string(),
                ],
                min_length: 4,
                pattern: r"([A-Z0-9][-]?[A-Z0-9]{3,20})\b".to_string(),
                confidence: 0.90,
            },
        );
        configs
    }
    
    fn clean_id(value: &str) -> String {
        value
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_uppercase())
            .collect()
    }
}

impl Default for IDExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl IDExtractor {
    fn compile_patterns(&mut self) {
        self.configs = Self::get_configs();
        self.patterns.clear();
        
        for (entity_type, config) in &self.configs {
            let mut patterns = Vec::new();
            for label in &config.labels {
                let full_pattern = format!("{}{}", label, config.pattern);
                if let Ok(re) = Regex::new(&full_pattern) {
                    patterns.push(re);
                }
            }
            self.patterns.insert(entity_type.clone(), patterns);
        }
    }
}

impl EntityExtractor for IDExtractor {
    fn name(&self) -> &str {
        "IDExtractor"
    }

    fn supported_types(&self) -> Vec<EntityType> {
        vec![EntityType::Id]
    }

    fn extract(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();
        let mut detected = HashSet::new();
        
        for (entity_type, patterns) in &self.patterns {
            let config = match self.configs.get(entity_type) {
                Some(c) => c,
                None => continue,
            };
            
            for pattern in patterns {
                for caps in pattern.captures_iter(text) {
                    if let Some(matched) = caps.get(1) {
                        let value = matched.as_str().to_string();
                        if detected.contains(&value) {
                            continue;
                        }
                        
                        let cleaned = Self::clean_id(&value);
                        if cleaned.len() >= config.min_length {
                            detected.insert(value.clone());
                            entities.push(Entity {
                                entity_type: entity_type.clone(),
                                value,
                                start: matched.start(),
                                end: matched.end(),
                                confidence: config.confidence,
                                placeholder: None,
                                metadata: HashMap::new(),
                            });
                        }
                    }
                }
            }
        }
        
        entities
    }
}