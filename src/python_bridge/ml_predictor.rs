use crate::schemas::{Entity, PrivacyDecision, DecisionType};
use crate::policy_engine::ContextBuilder;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};
use std::collections::HashMap;
use log;

pub struct MLPredictor {
    py_predictor: Option<PyObject>,
    context_builder: ContextBuilder,
    threshold: f32,
    loaded: bool,
}

impl MLPredictor {
    pub fn new() -> Self {
        let mut predictor = Self {
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
            
            let locals = PyDict::new(py);
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
    
    pub fn predict_single(&self, entity: &Entity, original_text: &str, intent: Option<&str>) -> PrivacyDecision {
        let context = self.context_builder.build(entity, original_text, intent);
        let (should_mask, confidence) = if let Some(predictor) = &self.py_predictor {
            Python::with_gil(|py| {
                let result = predictor.call_method(py, "predict", (context.clone(),), None);
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
            })
        } else {
            (true, 0.5)
        };
        
        let decision = if should_mask {
            DecisionType::Mask
        } else {
            DecisionType::Keep
        };
        
        PrivacyDecision::new(
            entity.clone(),
            decision,
            confidence,
            context,
        )
    }
    
    pub fn predict_batch(&self, entities: &[Entity], original_text: &str, intent: Option<&str>) -> Vec<PrivacyDecision> {
        let contexts: Vec<String> = entities.iter()
            .map(|e| self.context_builder.build(e, original_text, intent))
            .collect();
        
        if let Some(predictor) = &self.py_predictor {
            Python::with_gil(|py| {
                let contexts_list = PyList::new(py, contexts.iter());
                let result = predictor.call_method(py, "predict_batch", (contexts_list,), None);
                
                if let Ok(decisions) = result {
                    if let Ok(list) = decisions.downcast::<PyList>(py) {
                        let mut results = Vec::new();
                        for (i, item) in list.iter().enumerate() {
                            let (should_mask, confidence) = if let Ok(should_mask) = item.getattr(py, "should_mask") {
                                if let Ok(mask) = should_mask.extract::<bool>(py) {
                                    if let Ok(conf) = item.getattr(py, "confidence") {
                                        if let Ok(confidence) = conf.extract::<f32>(py) {
                                            (mask, confidence)
                                        } else {
                                            (mask, 0.5)
                                        }
                                    } else {
                                        (mask, 0.5)
                                    }
                                } else {
                                    (true, 0.5)
                                }
                            } else {
                                (true, 0.5)
                            };
                            
                            let decision = if should_mask {
                                DecisionType::Mask
                            } else {
                                DecisionType::Keep
                            };
                            
                            results.push(PrivacyDecision::new(
                                entities[i].clone(),
                                decision,
                                confidence,
                                contexts[i].clone(),
                            ));
                        }
                        return results;
                    }
                }
                
                entities.iter()
                    .enumerate()
                    .map(|(i, e)| {
                        PrivacyDecision::new(
                            e.clone(),
                            DecisionType::Mask,
                            0.5,
                            contexts[i].clone(),
                        )
                    })
                    .collect()
            })
        } else {
            entities.iter()
                .enumerate()
                .map(|(i, e)| {
                    PrivacyDecision::new(
                        e.clone(),
                        DecisionType::Mask,
                        0.5,
                        contexts[i].clone(),
                    )
                })
                .collect()
        }
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
