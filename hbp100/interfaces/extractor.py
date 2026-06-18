"""
Interface for entity extractors.
"""

from abc import ABC, abstractmethod
from typing import List

from hbp100.schemas.entity import Entity, EntityType


class EntityExtractorInterface(ABC):
    """
    Interface for entity extractors.

    All extractors must implement this interface.
    Extractors should only find entities and return structured Entity objects.
    They MUST NOT make privacy decisions.
    """

    @abstractmethod
    def extract(self, text: str) -> List[Entity]:
        """
        Extract entities from text.

        Args:
            text: Input text to analyze

        Returns:
            List of Entity objects

        Raises:
            ValueError: If text is empty or invalid
        """
        pass

    @abstractmethod
    def supports(self, entity_type: EntityType) -> bool:
        """
        Check if this extractor supports a given entity type.

        Args:
            entity_type: Entity type to check

        Returns:
            True if supported, False otherwise
        """
        pass

    @property
    @abstractmethod
    def supported_types(self) -> List[EntityType]:
        """
        List of entity types this extractor supports.

        Returns:
            List of EntityType enums
        """
        pass