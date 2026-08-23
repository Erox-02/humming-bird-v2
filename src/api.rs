use crate::core::Engine;
use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass]
pub struct HBP100 {
    engine: Engine,
}

#[pymethods]
impl HBP100 {
    #[new]
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }
    
    #[pyo3(signature = (text, intent=None))]
    pub fn process(&mut self, text: &str, intent: Option<&str>) -> PyResult<Py<PyDict>> {
        let result = self.engine.process(text, intent);
        
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            dict.set_item("original_text", result.original_text)?;
            dict.set_item("masked_text", result.masked_text)?;
            dict.set_item("metadata", result.metadata)?;
            dict.set_item("has_pii", result.has_pii)?;
            let entities: Vec<HashMap<String, String>> = result.entities
                .iter()
                .map(|e| {
                    let mut map = HashMap::new();
                    map.insert("type".to_string(), format!("{:?}", e.entity_type));
                    map.insert("value".to_string(), e.value.clone());
                    map.insert("start".to_string(), e.start.to_string());
                    map.insert("end".to_string(), e.end.to_string());
                    map.insert("confidence".to_string(), e.confidence.to_string());
                    map
                })
                .collect();
            dict.set_item("entities", entities)?;
            let decisions: Vec<HashMap<String, String>> = result.decisions
                .iter()
                .map(|d| {
                    let mut map = HashMap::new();
                    map.insert("entity_type".to_string(), format!("{:?}", d.entity.entity_type));
                    map.insert("entity_value".to_string(), d.entity.value.clone());
                    map.insert("decision".to_string(), format!("{:?}", d.decision));
                    map.insert("confidence".to_string(), d.confidence.to_string());
                    map
                })
                .collect();
            dict.set_item("decisions", decisions)?;
            
            Ok(dict.into())
        })
    }
    pub fn restore(&self, text: &str) -> String {
        self.engine.restore(text)
    }
    pub fn restore_with_metadata(&self, text: &str, metadata: HashMap<String, String>) -> String {
        self.engine.restore_with_metadata(text, metadata)
    }
    pub fn validate_response(&self, response: &str) -> (bool, Option<String>) {
        self.engine.validate_response(response)
    }
    pub fn reset(&mut self) {
        self.engine.reset();
    }
    pub fn metadata(&self) -> HashMap<String, String> {
        self.engine.metadata()
    }   
    pub fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

impl Default for HBP100 {
    fn default() -> Self {
        Self::new()
    }
}