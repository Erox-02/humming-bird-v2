from dataclasses import dataclass, field
from typing import Dict, List, Any

from hbp100.schemas.entity import Entity
from hbp100.schemas.decision import PrivacyDecision


@dataclass
class ProcessResult:
    """
    Result from processing text through the privacy pipeline.

    This is the main result object returned by HBP100.process().
    """

    original_text: str
    masked_text: str
    metadata: Dict[str, str] = field(default_factory=dict)
    entities: List[Entity] = field(default_factory=list)
    decisions: List[PrivacyDecision] = field(default_factory=list)
    has_pii: bool = False

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for serialization."""
        return {
            "original_text": self.original_text,
            "masked_text": self.masked_text,
            "metadata": self.metadata,
            "has_pii": self.has_pii,
            "entities": [e.to_dict() for e in self.entities],
            "decisions": [d.to_dict() for d in self.decisions],
        }