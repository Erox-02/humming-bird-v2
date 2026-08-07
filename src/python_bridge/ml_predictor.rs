use crate::schemas::{Entity, PrivacyDecision, DecisionType};
use crate::policy_engine::ContextBuilder;
use std::collections::HashMap;
use log;

#[cfg(feature = "python-bridge")]
use pyo3::prelude::*;

pub struct MLPredictor {
    #[cfg(feature = "python-bridge")]
    py_predictor: Option<pyo3::PyObject>,
    context_builder: ContextBuilder,
    threshold: f32,
    loaded: bool,
}

impl MLPredictor {
    pub fn new() -> Self {
        let mut predictor = Self {
            #[cfg(feature = "python-bridge")]
            py_predictor: None,
            context_builder: ContextBuilder::new(),
            threshold: 0.5,
            loaded: false,
        };
        predictor.load_assets();
        predictor
    }
    
    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }
    
    pub fn load_assets(&mut self) -> bool {
        if self.loaded {
            return true;
        }
        
        #[cfg(feature = "python-bridge")]
        {
            Python::with_gil(|py| {
                let result = py.run(
                    r#"
import sys
sys.path.insert(0, '.')

try:
    from hbp100.policy_engine.predictor import PrivacyPredictor
    _predictor = PrivacyPredictor()
    _predictor.load_assets()
except ImportError as e:
    print(f"Error loading predictor: {e}")
    _predictor = None
"#,
                    None,
                    None,
                );
                
                if result.is_err() {
                    log::error!("Failed to import Python predictor");
                    return false;
                }
                
                if let Ok(globals) = py.eval("globals()", None, None) {
                    if let Ok(predictor) = globals.get_item("_predictor") {
                        if !predictor.is_none() {
                            self.py_predictor = Some(predictor.to_object(py));
                            self.loaded = true;
                            log::info!("Python ML predictor loaded successfully");
                            return true;
                        }
                    }
                }
                
                log::error!("Failed to initialize Python predictor");
                false
            })
        }
        
        #[cfg(not(feature = "python-bridge"))]
        {
            log::warn!("Python bridge not enabled, using fallback mode");
            false
        }
    }
    
    pub fn predict(&self, context: &str) -> (bool, f32) {
        #[cfg(feature = "python-bridge")]
        {
            if let Some(predictor) = &self.py_predictor {
                return Python::with_gil(|py| {
                    let result = predictor.call_method(py, "predict", (context,), None);
                    if let Ok(decision) = result {
                        if let Ok(should_mask) = decision.getattr(py, "should_mask") {
                            if let Ok(mask) = should_mask.extract::<bool>(py) {
                                if let Ok(conf) = decision.getattr(py, "confidence") {
                                    if let Ok(confidence) = conf.extract::<f32>(py) {
                                        return (mask, confidence);
                                    }
                                }
                            }
                        }
                    }
                    (true, 0.5)
                });
            }
        }
        
        (true, 0.5)
    }
    
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }
    
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
    }
}

impl Default for MLPredictor {
    fn default() -> Self {
        Self::new()
    }
}
