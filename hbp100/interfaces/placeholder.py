from abc import ABC, abstractmethod
from typing import Dict, Optional

from hbp100.schemas.entity import Entity


class PlaceholderEngineInterface(ABC):
    """
    Interface for placeholder generation and management.
    """

    @abstractmethod
    def generate(self, entity: Entity) -> str:
        """
        Generate a placeholder for an entity.

        Args:
            entity: Entity to generate placeholder for

        Returns:
            Placeholder string
        """
        pass

    @abstractmethod
    def get_metadata(self) -> Dict[str, str]:
        """
        Get all placeholder-entity mappings as metadata.

        Returns:
            Dictionary mapping placeholders to original values
        """
        pass

    @abstractmethod
    def reset(self) -> None:
        """Reset all state."""
        pass