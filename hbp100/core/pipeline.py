import logging
from typing import Dict, Any, Optional, List, Tuple
from dataclasses import dataclass, field

from hbp100.schemas.entity import Entity
from hbp100.schemas.decision import PrivacyDecision, DecisionType
from hbp100.extractors.manager import ExtractorManager
from hbp100.policy_engine.predictor import PrivacyPredictor
from hbp100.placeholders.generator import PlaceholderGenerator
from hbp100.placeholders.validator import PlaceholderValidator
from hbp100.placeholders.restore import PlaceholderRestorer
from hbp100.utils.logger import get_logger

logger = get_logger(__name__)


@dataclass
class PipelineResult:
    original_text: str
    masked_text: str
    metadata: Dict[str, str] = field(default_factory=dict)
    entities: List[Entity] = field(default_factory=list)
    decisions: List[PrivacyDecision] = field(default_factory=list)
    has_pii: bool = False

    def to_dict(self) -> Dict[str, Any]:
        return {
            "original_text": self.original_text,
            "masked_text": self.masked_text,
            "metadata": self.metadata,
            "entities": [e.to_dict() for e in self.entities],
            "decisions": [d.to_dict() for d in self.decisions],
            "has_pii": self.has_pii,
        }


class PrivacyPipeline:
    def __init__(
        self,
        model_path: Optional[str] = None,
        vectorizer_path: Optional[str] = None,
        lazy_load: bool = True,
    ):
        self.extractor_manager = ExtractorManager()
        self.predictor = PrivacyPredictor(
            model_path=model_path,
            vectorizer_path=vectorizer_path,
        )
        self.generator = PlaceholderGenerator()
        self.validator = PlaceholderValidator()
        self.restorer = PlaceholderRestorer()

        if not lazy_load:
            self.load_assets()

        self._initialized = True
        logger.info("Privacy pipeline initialized")

    def load_assets(self) -> bool:
        return self.predictor.load_assets()

    def process(
        self,
        text: str,
        intent: Optional[str] = None,
        return_decisions: bool = False,
    ) -> PipelineResult:
        if not text or not text.strip():
            raise ValueError("Input text cannot be empty")

        self.generator.reset_all()
        self.validator.reset()
        self.restorer.reset()

        logger.info(f"Processing text (length: {len(text)} chars)")

        entities = self._extract_entities(text)
        logger.info(f"Extracted {len(entities)} entities")

        if not entities:
            return PipelineResult(
                original_text=text,
                masked_text=text,
                metadata={},
                entities=[],
                decisions=[],
                has_pii=False,
            )

        decisions = self._predict_decisions(entities, text, intent)
        logger.info(f"Predicted {len(decisions)} decisions")

        masked_text, metadata = self._apply_masking(text, entities, decisions)
        logger.info(f"Masked {len(metadata)} entities")

        self.validator.update_allowed(metadata)
        self.restorer.update_metadata(metadata)

        has_pii = any(d.should_mask for d in decisions)
        decisions_to_return = decisions if return_decisions else []

        return PipelineResult(
            original_text=text,
            masked_text=masked_text,
            metadata=metadata,
            entities=entities,
            decisions=decisions_to_return,
            has_pii=has_pii,
        )

    def _extract_entities(self, text: str) -> List[Entity]:
        return self.extractor_manager.extract_all(text)

    def _predict_decisions(
        self,
        entities: List[Entity],
        original_text: str,
        intent: Optional[str] = None,
    ) -> List[PrivacyDecision]:
        if not self.predictor.is_loaded:
            logger.warning("Predictor not loaded, using fallback decisions")
            return self._fallback_decisions(entities)

        return self.predictor.predict_batch(entities, original_text, intent)

    def _fallback_decisions(self, entities: List[Entity]) -> List[PrivacyDecision]:
        return [
            PrivacyDecision(
                entity=entity,
                decision=DecisionType.MASK,
                confidence=0.50,
                context_string=f"fallback [SEP] {entity.type.value} [SEP] {entity.value}",
            )
            for entity in entities
        ]

    def _apply_masking(
        self,
        text: str,
        entities: List[Entity],
        decisions: List[PrivacyDecision],
    ) -> Tuple[str, Dict[str, str]]:
        decision_map = {d.entity: d for d in decisions}

        sorted_entities = sorted(entities, key=lambda e: e.start, reverse=True)

        masked = text

        for entity in sorted_entities:
            decision = decision_map.get(entity)
            if decision and decision.should_mask:
                placeholder = self.generator.generate(entity)
                masked = (
                    masked[: entity.start] + placeholder + masked[entity.end :]
                )

        metadata = self.generator.get_metadata()
        return masked, metadata

    def restore_placeholders(
        self,
        text: str,
        metadata: Optional[Dict[str, str]] = None,
    ) -> str:
        return self.restorer.restore(text, metadata)

    def validate_response(self, response: str, metadata: Dict[str, str]) -> Tuple[bool, Optional[str]]:
        return self.validator.validate(response, metadata)

    def reset(self):
        self.generator.reset_all()
        self.validator.reset()
        self.restorer.reset()
        logger.info("Pipeline reset")

    @property
    def is_loaded(self) -> bool:
        return self.predictor.is_loaded
