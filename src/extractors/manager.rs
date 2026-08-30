use crate::extractors::{
    AddressExtractor, DateExtractor, EmailExtractor, PhoneExtractor,
    IDExtractor, NameExtractor, MedicalExtractor,
};
use crate::extractors::config::{ConfigurableExtractor, ExtractorConfig};
use crate::interfaces::EntityExtractor;
use crate::schemas::{Entity, EntityType};
use std::collections::{HashMap, HashSet};
use log;

pub struct ExtractorManager {
    extractors: Vec<Box<dyn EntityExtractor>>,
    type_map: HashMap<EntityType, usize>,
    enabled: HashSet<String>,
}

impl ExtractorManager {
    pub fn new() -> Self {
        let mut manager = Self {
            extractors: Vec::new(),
            type_map: HashMap::new(),
            enabled: HashSet::new(),
        };
        manager.register_default_extractors();
        manager.build_type_map();
        log::info!("Extractor manager initialized with {} extractors", manager.extractors.len());
        manager
    }
    
    fn register_default_extractors(&mut self) {
        self.register(Box::new(NameExtractor::new()));
        self.register(Box::new(PhoneExtractor::new()));
        self.register(Box::new(EmailExtractor::new()));
        self.register(Box::new(DateExtractor::new()));
        self.register(Box::new(AddressExtractor::new()));
        self.register(Box::new(IDExtractor::new()));
        self.register(Box::new(MedicalExtractor::new()));
    }

    fn build_type_map(&mut self) {
        self.type_map.clear();
        for (idx, extractor) in self.extractors.iter().enumerate() {
            for entity_type in extractor.supported_types() {
                self.type_map.insert(entity_type, idx);
            }
        }
    }

    pub fn register(&mut self, extractor: Box<dyn EntityExtractor>) {
        let name = extractor.name().to_string();
        for entity_type in extractor.supported_types() {
            self.type_map.insert(entity_type, self.extractors.len());
        }
        self.enabled.insert(name.clone());
        self.extractors.push(extractor);
        log::info!("Registered extractor: {}", name);
    }

    pub fn add_config_extractor(&mut self, config: ExtractorConfig) -> Result<(), String> {
        let extractor = ConfigurableExtractor::new(config)?;
        self.register(Box::new(extractor));
        Ok(())
    }

    pub fn add_config_from_json(&mut self, json: &str) -> Result<(), String> {
        let config: ExtractorConfig = serde_json::from_str(json)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        self.add_config_extractor(config)
    }

    pub fn add_config_from_file(&mut self, path: &str) -> Result<(), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let config: ExtractorConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        
        self.add_config_extractor(config)
    }

    pub fn enable_extractor(&mut self, name: &str) -> bool {
        self.enabled.insert(name.to_string())
    }

    pub fn disable_extractor(&mut self, name: &str) -> bool {
        self.enabled.remove(name)
    }

    pub fn is_enabled(&self, name: &str) -> bool {
        self.enabled.contains(name)
    }

    pub fn list_extractors(&self) -> Vec<String> {
        self.extractors
            .iter()
            .map(|e| e.name().to_string())
            .collect()
    }

    pub fn list_enabled(&self) -> Vec<String> {
        self.enabled.iter().cloned().collect()
    }

    pub fn extract_all(&self, text: &str) -> Vec<Entity> {
        let mut all_entities = Vec::new();
        let mut detected_values = HashSet::new();
        
        for extractor in &self.extractors {
            if !self.enabled.contains(extractor.name()) {
                continue;
            }
            
            let entities = extractor.extract(text);
            for entity in entities {
                if !detected_values.contains(&entity.value) {
                    all_entities.push(entity.clone());
                    detected_values.insert(entity.value);
                }
            }
        }
        
        all_entities.sort_by(|a, b| {
            a.start.cmp(&b.start)
                .then_with(|| (b.end - b.start).cmp(&(a.end - a.start)))
        });
        
        let mut filtered = Vec::new();
        for entity in all_entities {
            let overlaps = filtered.iter().any(|kept: &Entity| {
                !(entity.end <= kept.start || entity.start >= kept.end)
            });
            if !overlaps {
                filtered.push(entity);
            }
        }
        
        filtered
    }

    pub fn extract_by_type(&self, text: &str, entity_type: EntityType) -> Vec<Entity> {
        if let Some(&idx) = self.type_map.get(&entity_type) {
            if let Some(extractor) = self.extractors.get(idx) {
                if self.enabled.contains(extractor.name()) {
                    return extractor.extract(text)
                        .into_iter()
                        .filter(|e| e.entity_type == entity_type)
                        .collect();
                }
            }
        }
        log::warn!("No extractor found for entity type: {:?}", entity_type);
        Vec::new()
    }

    pub fn clear_extractors(&mut self) {
        self.extractors.clear();
        self.type_map.clear();
        self.enabled.clear();
    }

    pub fn reset_to_defaults(&mut self) {
        self.clear_extractors();
        self.register_default_extractors();
        self.build_type_map();
    }

    pub fn get_type_map(&self) -> &HashMap<EntityType, usize> {
        &self.type_map
    }

    pub fn extractor_count(&self) -> usize {
        self.extractors.len()
    }
}

impl Default for ExtractorManager {
    fn default() -> Self {
        Self::new()
    }
}