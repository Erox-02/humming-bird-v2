from dataclasses import dataclass
from enum import Enum
from typing import Optional, Dict, Any

from hbp100.schemas.entity import Entity


class DecisionType(str, Enum):
    """Privacy decision types."""

    KEEP = "KEEP"  # Keep entity visible
    MASK = "MASK"  # Mask entity with placeholder


@dataclass
class PrivacyDecision:
    """
    Decision from the privacy policy engine.

    Attributes:
        entity: The entity being decided on
        decision: KEEP or MASK
        confidence: Model confidence (0.0-1.0)
        context_string: The string used for prediction
        model_prediction: Raw model output
        reasoning: Optional human-readable reasoning
    """

    entity: Entity
    decision: DecisionType
    confidence: float
    context_string: str
    model_prediction: Any = None
    reasoning: Optional[str] = None

    def __post_init__(self):
        """Validate confidence."""
        if not 0.0 <= self.confidence <= 1.0:
            raise ValueError("Confidence must be between 0.0 and 1.0")

    @property
    def should_mask(self) -> bool:
        """Check if entity should be masked."""
        return self.decision == DecisionType.MASK

    @property
    def should_keep(self) -> bool:
        """Check if entity should be kept."""
        return self.decision == DecisionType.KEEP

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for serialization."""
        return {
            "entity_type": self.entity.type.value,
            "entity_value": self.entity.value,
            "decision": self.decision.value,
            "confidence": self.confidence,
            "should_mask": self.should_mask,
        }