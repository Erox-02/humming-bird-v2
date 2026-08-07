use crate::schemas::{Entity, Placeholder};
use std::collections::HashMap;

pub trait PlaceholderEngine: Send + Sync {
    fn generate(&mut self, entity: &Entity) -> String;
    
    fn get_metadata(&self) -> HashMap<String, String>;
    
    fn get_value(&self, placeholder: &str) -> Option<String>;
    
    fn reset(&mut self);
    
    fn is_valid_placeholder(&self, text: &str) -> bool;
}
