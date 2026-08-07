from dataclasses import dataclass, field
from typing import Optional, Dict, Any
from enum import Enum


class EntityType(str, Enum):
    NAME = "NAME"
    PHYSICIAN = "PHYSICIAN"
    PATIENT_ID = "PATIENT_ID"
    EMAIL = "EMAIL"
    PHONE = "PHONE"
    ADDRESS = "ADDRESS"
    MRN = "MRN"
    CASE_ID = "CASE_ID"
    POLICY_NUMBER = "POLICY_NUMBER"
    DATE = "DATE"
    DOB = "DOB"
    SSN = "SSN"
    HOSPITAL = "HOSPITAL"
    PASSPORT = "PASSPORT"


@dataclass
class Entity:
    type: EntityType
    value: str
    start: int
    end: int
    confidence: float = 1.0
    placeholder: Optional[str] = None
    metadata: Dict[str, Any] = field(default_factory=dict)

    def __post_init__(self):
        if self.end <= self.start:
            raise ValueError("End position must be greater than start")
        if not 0.0 <= self.confidence <= 1.0:
            raise ValueError("Confidence must be between 0.0 and 1.0")

    @property
    def text(self) -> str:
        return self.value

    @property
    def length(self) -> int:
        return len(self.value)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "type": self.type.value,
            "value": self.value,
            "start": self.start,
            "end": self.end,
            "confidence": self.confidence,
            "placeholder": self.placeholder,
            "metadata": self.metadata,
        }

    def __hash__(self) -> int:
        return hash((self.type, self.value))

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Entity):
            return False
        return self.type == other.type and self.value == other.value
