use crate::interfaces::EntityExtractor;
use crate::schemas::{Entity, EntityType};
use regex::Regex;
use std::collections::HashSet;

pub trait BaseExtractor: EntityExtractor {
    fn compile_patterns(&mut self);
    
    fn extract_matches(
        &self,
        text: &str,
        patterns: &[Regex],
        entity_type: EntityType,
        confidence: f32,
    ) -> Vec<Entity> {
        let mut entities = Vec::new();
        let mut detected = HashSet::new();
        
        for pattern in patterns {
            for caps in pattern.captures_iter(text) {
                if let Some(matched) = caps.get(0) {
                    let value = matched.as_str().to_string();
                    if !detected.contains(&value) {
                        detected.insert(value.clone());
                        entities.push(Entity::new(
                            entity_type,
                            value,
                            matched.start(),
                            matched.end(),
                            confidence,
                        ));
                    }
                }
            }
        }
        entities
    }
    
    fn extract_group_matches(
        &self,
        text: &str,
        patterns: &[Regex],
        entity_type: EntityType,
        group: usize,
        confidence: f32,
        min_length: usize,
    ) -> Vec<Entity> {
        let mut entities = Vec::new();
        let mut detected = HashSet::new();
        
        for pattern in patterns {
            for caps in pattern.captures_iter(text) {
                if let Some(matched) = caps.get(group) {
                    let value = matched.as_str().trim().to_string();
                    if !detected.contains(&value) && value.len() >= min_length {
                        detected.insert(value.clone());
                        entities.push(Entity::new(
                            entity_type,
                            value,
                            matched.start(),
                            matched.end(),
                            confidence,
                        ));
                    }
                }
            }
        }
        entities
    }
    
    fn is_valid_name_format(&self, value: &str) -> bool {
        if value.is_empty() {
            return false;
        }
        let parts: Vec<&str> = value.split_whitespace().collect();
        for part in parts {
            if part.is_empty() {
                return false;
            }
            let chars: Vec<char> = part.chars().collect();
            if !chars[0].is_uppercase() {
                return false;
            }
            for c in &chars[1..] {
                if !c.is_alphabetic() {
                    return false;
                }
            }
        }
        true
    }
    
    fn is_all_caps(&self, value: &str) -> bool {
        value
            .chars()
            .all(|c| !c.is_alphabetic() || c.is_uppercase())
    }
    
    fn has_context_keyword(&self, text: &str, start: usize, end: usize, keywords: &[&str]) -> bool {
        let window_start = start.saturating_sub(200);
        let window_end = (end + 200).min(text.len());
        let window = &text[window_start..window_end];
        let window_lower = window.to_lowercase();
        
        keywords.iter().any(|kw| window_lower.contains(&kw.to_lowercase()))
    }
}
