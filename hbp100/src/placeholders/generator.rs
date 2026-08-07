use crate::schemas::Entity;
use crate::core::MetadataVault;
use std::collections::HashMap;
use regex::Regex;
use log;

pub struct PlaceholderGenerator {
    counters: HashMap<String, usize>,
    vault: MetadataVault,
    pattern: Regex,
}

impl PlaceholderGenerator {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
            vault: MetadataVault::new(),
            pattern: Regex::new(r"\[([A-Z_]+)_(\d+)\]").unwrap(),
        }
    }
    
    pub fn generate(&mut self, entity: &Entity) -> String {
        let entity_type = entity.entity_type.as_str();
        let counter = self.counters.entry(entity_type.to_string()).or_insert(0);
        *counter += 1;
        let placeholder = format!("[{}_{}]", entity_type, counter);
        
        self.vault.set(placeholder.clone(), entity.value.clone());
        log::debug!("Generated placeholder {} for {:?}", placeholder, entity.entity_type);
        
        placeholder
    }
    
    pub fn get_metadata(&self) -> HashMap<String, String> {
        self.vault.get_all()
    }
    
    pub fn get_counter(&self, entity_type: &str) -> usize {
        self.counters.get(entity_type).copied().unwrap_or(0)
    }
    
    pub fn reset(&mut self) {
        self.counters.clear();
        log::debug!("Placeholder generator counters reset");
    }
    
    pub fn reset_all(&mut self) {
        self.counters.clear();
        self.vault.clear();
        log::debug!("Placeholder generator and vault reset");
    }
    
    pub fn is_valid_placeholder(&self, text: &str) -> bool {
        self.pattern.is_match(text)
    }
    
    pub fn extract_placeholder_type(&self, placeholder: &str) -> Option<String> {
        if let Some(caps) = self.pattern.captures(placeholder) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }
    
    pub fn extract_placeholder_count(&self, placeholder: &str) -> Option<usize> {
        if let Some(caps) = self.pattern.captures(placeholder) {
            caps.get(2).and_then(|m| m.as_str().parse().ok())
        } else {
            None
        }
    }
}

impl Default for PlaceholderGenerator {
    fn default() -> Self {
        Self::new()
    }
}
