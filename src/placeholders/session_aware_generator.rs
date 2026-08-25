use crate::Session;
use regex::Regex;
use std::sync::OnceLock;

static PLACEHOLDER_PATTERN: OnceLock<Regex> = OnceLock::new();

fn get_pattern() -> &'static Regex {
    PLACEHOLDER_PATTERN.get_or_init(|| {
        Regex::new(r"\[([A-Z_]+)_(\d+)\]").unwrap()
    })
}

pub struct SessionAwareGenerator<'a> {
    session: &'a mut Session,
}

impl<'a> SessionAwareGenerator<'a> {
    pub fn new(session: &'a mut Session) -> Self {
        Self {
            session,
        }
    }

    pub fn generate(&mut self, entity_type: &str, value: &str) -> String {
        let counter = self.session.get_next_counter(entity_type);
        let placeholder = format!("[{}_{}]", entity_type, counter);
        
        let mut metadata = std::collections::HashMap::new();
        metadata.insert(placeholder.clone(), value.to_string());
        self.session.update_metadata(metadata);
        
        placeholder
    }

    pub fn is_valid_placeholder(&self, text: &str) -> bool {
        get_pattern().is_match(text)
    }

    pub fn extract_placeholder_type(&self, placeholder: &str) -> Option<String> {
        if let Some(caps) = get_pattern().captures(placeholder) {
            caps.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

    pub fn extract_placeholder_count(&self, placeholder: &str) -> Option<usize> {
        if let Some(caps) = get_pattern().captures(placeholder) {
            caps.get(2).and_then(|m| m.as_str().parse().ok())
        } else {
            None
        }
    }
}