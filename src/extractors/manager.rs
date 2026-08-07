use crate::extractors::{
    AddressExtractor, DateExtractor, EmailExtractor, PhoneExtractor,
    IDExtractor, NameExtractor, HospitalExtractor,
};
use crate::interfaces::EntityExtractor;
use crate::schemas::{Entity, EntityType};
use std::collections::HashMap;
use log;

pub struct ExtractorManager {
    extractors: Vec<Box<dyn EntityExtractor>>,
    type_map: HashMap<EntityType, usize>,
}

impl ExtractorManager {
    pub fn new() -> Self {
        let mut manager = Self {
            extractors: Vec::new(),
            type_map: HashMap::new(),
        };
        manager.register_default_extractors();
        manager.build_type_map();
        log::info!("Extractor manager initialized with {} extractors", manager.extractors.len());
        manager
    }
    
    fn register_default_extractors(&mut self) {
        self.extractors.push(Box::new(NameExtractor::new()));
        self.extractors.push(Box::new(PhoneExtractor::new()));
        self.extractors.push(Box::new(EmailExtractor::new()));
        self.extractors.push(Box::new(DateExtractor::new()));
        self.extractors.push(Box::new(AddressExtractor::new()));
        self.extractors.push(Box::new(IDExtractor::new()));
        self.extractors.push(Box::new(HospitalExtractor::new()));
    }
    
    fn build_type_map(&mut self) {
        self.type_map.clear();
        for (idx, extractor) in self.extractors.iter().enumerate() {
            for entity_type in extractor.supported_types() {
                self.type_map.insert(entity_type, idx);
            }
        }
    }
    
    pub fn extract_all(&self, text: &str) -> Vec<Entity> {
        let mut all_entities = Vec::new();
        let mut detected_values = std::collections::HashSet::new();
        
        for extractor in &self.extractors {
            match extractor.extract(text) {
                entities => {
                    for entity in entities {
                        if !detected_values.contains(&entity.value) {
                            all_entities.push(entity.clone());
                            detected_values.insert(entity.value);
                        }
                    }
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
                return extractor.extract(text)
                    .into_iter()
                    .filter(|e| e.entity_type == entity_type)
                    .collect();
            }
        }
        log::warn!("No extractor found for entity type: {:?}", entity_type);
        Vec::new()
    }
    
    pub fn register(&mut self, extractor: Box<dyn EntityExtractor>) {
        for entity_type in extractor.supported_types() {
            self.type_map.insert(entity_type, self.extractors.len());
        }
        self.extractors.push(extractor);
        log::info!("Registered extractor");
    }
}

impl Default for ExtractorManager {
    fn default() -> Self {
        Self::new()
    }
}
