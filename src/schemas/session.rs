use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub metadata: HashMap<String, String>,
    pub counters: HashMap<String, usize>,
    pub created_at: u64,
    pub updated_at: u64,
    pub intent: Option<String>,
    pub is_active: bool,
}

impl Session {
    pub fn new(id: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id,
            metadata: HashMap::with_capacity(64),
            counters: HashMap::with_capacity(16),
            created_at: now,
            updated_at: now,
            intent: None,
            is_active: true,
        }
    }

    pub fn with_intent(mut self, intent: impl Into<String>) -> Self {
        self.intent = Some(intent.into());
        self
    }

    #[inline]
    pub fn update_metadata(&mut self, new_metadata: HashMap<String, String>) {
        self.metadata.extend(new_metadata);
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    #[inline]
    pub fn get_next_counter(&mut self, entity_type: &str) -> usize {
        let counter = self.counters.entry(entity_type.to_string()).or_insert(0);
        *counter += 1;
        *counter
    }

    #[inline]
    pub fn get_metadata(&self) -> HashMap<String, String> {
        self.metadata.clone()
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new(String::new())
    }
}