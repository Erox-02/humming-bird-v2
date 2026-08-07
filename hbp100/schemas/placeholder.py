from dataclasses import dataclass
from typing import Dict


@dataclass
class Placeholder:
    """
    Represents a placeholder with its original value.

    Attributes:
        placeholder: The placeholder string (e.g., "[NAME_1]")
        original_value: The original value being masked
        entity_type: Type of entity being masked
    """

    placeholder: str
    original_value: str
    entity_type: str

    def to_metadata(self) -> Dict[str, str]:
        """Convert to metadata format."""
        return {self.placeholder: self.original_value}