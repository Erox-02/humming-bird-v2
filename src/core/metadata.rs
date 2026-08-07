use std::collections::HashMap;
use std::sync::RwLock;

pub struct MetadataVault {
    metadata: RwLock<HashMap<String, String>>,
}

impl MetadataVault {
    pub fn new() -> Self {
        Self {
            metadata: RwLock::new(HashMap::new()),
        }
    }
    
    pub fn set(&self, placeholder: String, value: String) {
        let mut metadata = self.metadata.write().unwrap();
        metadata.insert(placeholder, value);
    }
    
    pub fn get(&self, placeholder: &str) -> Option<String> {
        let metadata = self.metadata.read().unwrap();
        metadata.get(placeholder).cloned()
    }
    
    pub fn update(&self, mappings: HashMap<String, String>) {
        let mut metadata = self.metadata.write().unwrap();
        metadata.extend(mappings);
    }
    
    pub fn get_all(&self) -> HashMap<String, String> {
        let metadata = self.metadata.read().unwrap();
        metadata.clone()
    }
    
    pub fn clear(&self) {
        let mut metadata = self.metadata.write().unwrap();
        metadata.clear();
    }
    
    pub fn len(&self) -> usize {
        let metadata = self.metadata.read().unwrap();
        metadata.len()
    }
    
    pub fn is_empty(&self) -> bool {
        let metadata = self.metadata.read().unwrap();
        metadata.is_empty()
    }
}

impl Default for MetadataVault {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static::lazy_static! {
    pub static ref GLOBAL_VAULT: MetadataVault = MetadataVault::new();
}
