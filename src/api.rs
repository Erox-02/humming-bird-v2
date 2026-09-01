use crate::core::Engine;
use crate::core::SessionManager;
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
    pub fn add_extractor(&mut self, config_json: &str) -> PyResult<()> {
        self.engine.add_config_extractor(config_json)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }
    pub fn add_extractor_from_file(&mut self, path: &str) -> PyResult<()> {
        self.engine.add_config_extractor_from_file(path)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }
    pub fn enable_extractor(&mut self, name: &str) -> bool {
        self.engine.enable_extractor(name)
    }
    pub fn disable_extractor(&mut self, name: &str) -> bool {
        self.engine.disable_extractor(name)
    }
    pub fn list_extractors(&self) -> Vec<String> {
        self.engine.list_extractors()
    }
    pub fn list_enabled(&self) -> Vec<String> {
        self.engine.list_enabled_extractors()
    }
    pub fn reset_extractors(&mut self) {
        self.engine.reset_extractors();
    }
}

impl Default for HBP100 {
    fn default() -> Self {
        Self::new()
    }
}

#[pyclass]
pub struct HBP100Session {
    session_mgr: SessionManager,
    session_id: String,
}

#[pymethods]
impl HBP100Session {
    #[new]
    #[pyo3(signature = (intent=None))]
    pub fn new(intent: Option<&str>) -> Self {
        let session_mgr = SessionManager::new();
        let session_id = session_mgr.create_session(intent);
        Self { session_mgr, session_id }
    }
    
    #[pyo3(signature = (engine, text))]
    pub fn process(&mut self, engine: &mut HBP100, text: &str) -> PyResult<Py<PyDict>> {
        let session_arc = self.session_mgr.get_session(&self.session_id)
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Session not found"))?;
        
        let intent;
        {
            let session = session_arc.read().unwrap();
            intent = session.intent.clone();
        }
        
        let mut session = session_arc.write().unwrap();
        let result = engine.engine.process_with_session(
            text,
            &mut session,
            intent.as_deref(),
        );
        
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
    
    pub fn restore(&mut self, engine: &mut HBP100, text: &str) -> PyResult<String> {
        let session_arc = self.session_mgr.get_session(&self.session_id)
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Session not found"))?;
        
        let session = session_arc.read().unwrap();
        let metadata = session.get_metadata();
        Ok(engine.engine.restore_with_metadata(text, metadata))
    }
    
    pub fn session_id(&self) -> String {
        self.session_id.clone()
    }
}