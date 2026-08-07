import os
import sys
from pathlib import Path

try:
    from hbp100_rs import _rust_extractors
except ImportError:
    import importlib.util
    spec = importlib.util.find_spec("_rust_extractors")
    if spec is None:
        raise ImportError(
            "Could not import _rust_extractors. "
            "Make sure the Rust library is built and installed."
        )
    from hbp100_rs import _rust_extractors

from hbp100.policy_engine.predictor import PrivacyPredictor
from hbp100.core.metadata import MetadataVault

EntityType = _rust_extractors.EntityType
Entity = _rust_extractors.Entity
DecisionType = _rust_extractors.DecisionType
PrivacyDecision = _rust_extractors.PrivacyDecision

class HBP100:
    def __init__(self, model_path=None, vectorizer_path=None, lazy_load=True):
        self._engine = _rust_extractors.HBP100()
        self._engine.with_predictor()
        
        self._predictor = PrivacyPredictor(
            model_path=model_path,
            vectorizer_path=vectorizer_path,
        )
        if not lazy_load:
            self._predictor.load_assets()
        
        self._metadata_vault = MetadataVault()
        self._loaded = not lazy_load
        
    def process(self, text, intent=None, return_entities=False):
        entities = self._engine.extract_entities(text)
        
        if not entities:
            return {
                "original_text": text,
                "masked_text": text,
                "metadata": {},
                "has_pii": False,
                "entities": [],
                "decisions": [],
            }
        
        contexts = self._build_contexts(entities, text, intent)
        
        decisions = self._predictor.predict_batch(entities, text, intent)
        
        masked_text, metadata = self._apply_masking(text, entities, decisions)
        
        self._metadata_vault.update(metadata)
        
        has_pii = any(d.decision == "MASK" for d in decisions)
        
        result = {
            "original_text": text,
            "masked_text": masked_text,
            "metadata": metadata,
            "has_pii": has_pii,
        }
        
        if return_entities:
            result["entities"] = [e.to_dict() for e in entities]
            result["decisions"] = [d.to_dict() for d in decisions]
        
        return result
    
    def _build_contexts(self, entities, text, intent):
        contexts = []
        for entity in entities:
            context_parts = []
            if intent:
                context_parts.append(intent)
            context_parts.append(entity.entity_type)
            context_parts.append(entity.value)
            
            start = max(0, entity.start - 100)
            end = min(len(text), entity.end + 100)
            context_parts.append(text[start:end])
            
            contexts.append(" [SEP] ".join(context_parts))
        
        return contexts
    
    def _apply_masking(self, text, entities, decisions):
        decision_map = {id(e): d for e, d in zip(entities, decisions)}
        
        sorted_entities = sorted(
            enumerate(entities),
            key=lambda x: x[1].end,
            reverse=True
        )
        
        masked = text
        metadata = {}
        
        for idx, entity in sorted_entities:
            if idx in decision_map and decision_map[idx].should_mask:
                placeholder = f"[{entity.entity_type}_{len(metadata) + 1}]"
                masked = masked[:entity.start] + placeholder + masked[entity.end:]
                metadata[placeholder] = entity.value
        
        return masked, metadata
    
    def restore(self, text):
        metadata = self._metadata_vault.get_all()
        return self._restore_with_metadata(text, metadata)
    
    def _restore_with_metadata(self, text, metadata):
        restored = text
        for placeholder, value in sorted(metadata.items(), key=lambda x: len(x[0]), reverse=True):
            restored = restored.replace(placeholder, value)
        return restored
    
    def reset(self):
        self._engine.reset()
        self._metadata_vault.clear()
    
    @property
    def metadata(self):
        return self._metadata_vault.get_all()
    
    @property
    def is_loaded(self):
        return self._loaded


def process_text(text, intent=None):
    hbp = HBP100()
    return hbp.process(text, intent)


__all__ = [
    "HBP100",
    "process_text",
    "EntityType",
    "Entity",
    "DecisionType",
    "PrivacyDecision",
]
