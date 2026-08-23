use crate::schemas::Entity;

pub struct ContextBuilder {
    separator: String,
    window_size: usize,
}
impl ContextBuilder {
    pub fn new() -> Self {
        Self {
            separator: " [SEP] ".to_string(),
            window_size: 100,
        }
    }
    pub fn build(&self, entity: &Entity, original_text: &str, intent: Option<&str>) -> String {
        let mut parts = Vec::new();
        
        if let Some(intent) = intent {
            parts.push(intent.trim().to_string());
        }       
        parts.push(entity.entity_type.as_str().to_string());
        parts.push(entity.value.clone());        
        let context = self.extract_surrounding_context(original_text, entity.start, entity.end);
        parts.push(context);
        let full_text = self.truncate_text(original_text, entity.start, entity.end);
        parts.push(full_text);
        parts.join(&self.separator)
    }
    fn extract_surrounding_context(&self, text: &str, start: usize, end: usize) -> String {
        let context_start = start.saturating_sub(self.window_size);
        let context_end = (end + self.window_size).min(text.len());        
        let mut parts = Vec::new();
        if context_start > 0 {
            parts.push("...".to_string());
        }
        parts.push(text[context_start..context_end].to_string());
        if context_end < text.len() {
            parts.push("...".to_string());
        }        
        parts.concat()
    }
    fn truncate_text(&self, text: &str, start: usize, end: usize) -> String {
        let max_len = 1000;
        if text.len() <= max_len {
            return text.to_string();
        }   
        let mut parts = Vec::new();
        if start > 100 {
            parts.push("...".to_string());
        }
        let context_start = start.saturating_sub(100);
        let context_end = (end + 100).min(text.len());
        parts.push(text[context_start..context_end].to_string());
        if end < text.len() - 100 {
            parts.push("...".to_string());
        }
        parts.concat()
    }
    pub fn batch_build(&self, entities: &[Entity], original_text: &str, intent: Option<&str>) -> Vec<String> {
        entities.iter()
            .map(|e| self.build(e, original_text, intent))
            .collect()
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}
