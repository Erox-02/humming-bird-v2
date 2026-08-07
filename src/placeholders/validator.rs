use std::collections::HashSet;
use regex::Regex;
use log;

pub struct PlaceholderValidator {
    allowed: HashSet<String>,
    pattern: Regex,
}

impl PlaceholderValidator {
    pub fn new() -> Self {
        Self {
            allowed: HashSet::new(),
            pattern: Regex::new(r"\[[A-Z_]+_\d+\]").unwrap(),
        }
    }
    
    pub fn validate(&self, response: &str) -> (bool, Option<String>) {
        if response.is_empty() {
            return (true, None);
        }
        
        let placeholders: Vec<String> = self.pattern
            .find_iter(response)
            .map(|m| m.as_str().to_string())
            .collect();
        
        if placeholders.is_empty() {
            return (true, None);
        }
        
        let invalid: Vec<String> = placeholders
            .into_iter()
            .filter(|ph| !self.allowed.contains(ph))
            .collect();
        
        if !invalid.is_empty() {
            let error = format!("Invalid placeholder(s) detected: {}", invalid.join(", "));
            log::warn!("{}", error);
            return (false, Some(error));
        }
        
        (true, None)
    }
    
    pub fn sanitize(&self, response: &str, replacement: &str) -> String {
        if response.is_empty() {
            return response.to_string();
        }
        
        let mut sanitized = response.to_string();
        for placeholder in self.pattern.find_iter(response) {
            let ph = placeholder.as_str();
            if !self.allowed.contains(ph) {
                sanitized = sanitized.replace(ph, replacement);
                log::debug!("Replaced unknown placeholder {}", ph);
            }
        }
        
        sanitized
    }
    
    pub fn update_allowed(&mut self, placeholders: impl IntoIterator<Item = String>) {
        for ph in placeholders {
            self.allowed.insert(ph);
        }
    }
    
    pub fn reset(&mut self) {
        self.allowed.clear();
        log::debug!("Placeholder validator reset");
    }
    
    pub fn get_allowed(&self) -> HashSet<String> {
        self.allowed.clone()
    }
}

impl Default for PlaceholderValidator {
    fn default() -> Self {
        Self::new()
    }
}
