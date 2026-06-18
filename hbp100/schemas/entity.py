from dataclasses import dataclass, field
from typing import Optional, Dict, Any
from enum import Enum


class EntityType(str, Enum):
    """Types of entities that can be extracted."""

    # Personal Information
    NAME = "NAME"
    PHYSICIAN = "PHYSICIAN"
    PATIENT_ID = "PATIENT_ID"

    # Contact
    EMAIL = "EMAIL"
    PHONE = "PHONE"
    ADDRESS = "ADDRESS"

    # Medical Identifiers
    MRN = "MRN"
    CASE_ID = "CASE_ID"
    POLICY_NUMBER = "POLICY_NUMBER"

    # Dates
    DATE = "DATE"
    DOB = "DOB"

    # Other
    SSN = "SSN"
    HOSPITAL = "HOSPITAL"


@dataclass
class Entity:
    """
    Represents an extracted entity with all metadata.

    Attributes:
        type: Entity type (NAME, PHONE, EMAIL, etc.)
        value: The actual text value
        start: Character start position (for precise replacement)
        end: Character end position (for precise replacement)
        confidence: Extraction confidence (0.0-1.0)
        placeholder: Generated placeholder (set later)
        metadata: Additional entity-specific metadata
    """

    type: EntityType
    value: str
    start: int
    end: int
    confidence: float = 1.0
    placeholder: Optional[str] = None
    metadata: Dict[str, Any] = field(default_factory=dict)

    def __post_init__(self):
        """Ensure end > start and valid confidence."""
        if self.end <= self.start:
            raise ValueError("End position must be greater than start")
        if not 0.0 <= self.confidence <= 1.0:
            raise ValueError("Confidence must be between 0.0 and 1.0")

    @property
    def text(self) -> str:
        """Alias for value."""
        return self.value

    @property
    def length(self) -> int:
        """Length of the entity value."""
        return len(self.value)

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for serialization."""
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
        """Hash based on value and type."""
        return hash((self.type, self.value))

    def __eq__(self, other: object) -> bool:
        """Equality based on value and type."""
        if not isinstance(other, Entity):
            return False
        return self.type == other.type and self.value == other.value