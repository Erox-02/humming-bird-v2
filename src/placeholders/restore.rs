use std::collections::HashMap;
use regex::Regex;
use log;

pub struct PlaceholderRestorer {
    vault: HashMap<String, String>,
    pattern: Regex,
}

impl PlaceholderRestorer {
    pub fn new() -> Self {
        Self {
            vault: HashMap::new(),
            pattern: Regex::new(r"\[[A-Z_]+_\d+\]").unwrap(),
        }
    }
    
    pub fn restore(&self, text: &str) -> String {
        if text.is_empty() || self.vault.is_empty() {
            return text.to_string();
        }
        
        let mut restored = text.to_string();
        
        // Sort placeholders by length descending to handle nested placeholders
        let mut placeholders: Vec<&String> = self.vault.keys().collect();
        placeholders.sort_by(|a, b| b.len().cmp(&a.len()));
        
        for placeholder in placeholders {
            if let Some(value) = self.vault.get(placeholder) {
                restored = restored.replace(placeholder, value);
            }
        }
        
        restored
    }
    
    pub fn restore_with_metadata(&self, text: &str, metadata: HashMap<String, String>) -> String {
        if text.is_empty() || metadata.is_empty() {
            return text.to_string();
        }
        
        let mut restored = text.to_string();
        let mut placeholders: Vec<&String> = metadata.keys().collect();
        placeholders.sort_by(|a, b| b.len().cmp(&a.len()));
        
        for placeholder in placeholders {
            if let Some(value) = metadata.get(placeholder) {
                restored = restored.replace(placeholder, value);
            }
        }
        
        restored
    }
    
    pub fn get_remaining_placeholders(&self, text: &str) -> Vec<String> {
        self.pattern
            .find_iter(text)
            .map(|m| m.as_str().to_string())
            .collect()
    }
    
    pub fn count_placeholders(&self, text: &str) -> usize {
        self.pattern.find_iter(text).count()
    }
    
    pub fn has_placeholders(&self, text: &str) -> bool {
        self.pattern.is_match(text)
    }
    
    pub fn update_metadata(&mut self, metadata: HashMap<String, String>) {
        self.vault = metadata;
    }
    
    pub fn reset(&mut self) {
        self.vault.clear();
        log::debug!("Placeholder restorer reset");
    }
}

impl Default for PlaceholderRestorer {
    fn default() -> Self {
        Self::new()
    }
}
