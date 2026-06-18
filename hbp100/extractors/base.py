import re
from abc import ABC, abstractmethod
from typing import List, Set, Optional, Pattern

from hbp100.schemas.entity import Entity, EntityType
from hbp100.utils.logger import get_logger

logger = get_logger(__name__)


class BaseExtractor(ABC):
    """
    Base class for all entity extractors.

    Provides common functionality and interface for entity extraction.
    """

    def __init__(self):
        """Initialize the extractor with compiled patterns."""
        self._patterns: List[Pattern] = []
        self._compile_patterns()

    @abstractmethod
    def _compile_patterns(self):
        """
        Compile regex patterns for extraction.

        Subclasses must implement this method to define their patterns.
        """
        pass

    @abstractmethod
    def extract(self, text: str) -> List[Entity]:
        """
        Extract entities from text.

        Args:
            text: Input text to analyze

        Returns:
            List of Entity objects
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

    def supports(self, entity_type: EntityType) -> bool:
        """
        Check if this extractor supports a given entity type.

        Args:
            entity_type: Entity type to check

        Returns:
            True if supported, False otherwise
        """
        return entity_type in self.supported_types

    def _validate_text(self, text: str) -> None:
        """
        Validate input text.

        Args:
            text: Input text to validate

        Raises:
            ValueError: If text is empty
        """
        if not text or not text.strip():
            raise ValueError("Input text cannot be empty")

    def _extract_matches(
        self,
        text: str,
        patterns: List[Pattern],
        entity_type: EntityType,
        confidence: float = 0.85,
    ) -> List[Entity]:
        """
        Extract entities using compiled patterns.

        Args:
            text: Input text
            patterns: List of compiled regex patterns
            entity_type: Entity type to assign
            confidence: Extraction confidence

        Returns:
            List of Entity objects
        """
        entities = []
        detected_values: Set[str] = set()

        for pattern in patterns:
            for match in pattern.finditer(text):
                value = match.group(0)
                if value in detected_values:
                    continue

                start, end = match.span()
                entity = Entity(
                    type=entity_type,
                    value=value,
                    start=start,
                    end=end,
                    confidence=confidence,
                )
                entities.append(entity)
                detected_values.add(value)

        return entities

    def _extract_group_matches(
        self,
        text: str,
        patterns: List[Pattern],
        entity_type: EntityType,
        group: int = 1,
        confidence: float = 0.85,
        min_length: int = 2,
    ) -> List[Entity]:
        """
        Extract entities using regex groups.

        Args:
            text: Input text
            patterns: List of compiled regex patterns with capture groups
            entity_type: Entity type to assign
            group: Group index to extract (default: 1)
            confidence: Extraction confidence
            min_length: Minimum length of extracted value

        Returns:
            List of Entity objects
        """
        entities = []
        detected_values: Set[str] = set()

        for pattern in patterns:
            for match in pattern.finditer(text):
                try:
                    value = match.group(group).strip()
                except IndexError:
                    continue

                if value in detected_values or len(value) < min_length:
                    continue

                start, end = match.span(group)
                entity = Entity(
                    type=entity_type,
                    value=value,
                    start=start,
                    end=end,
                    confidence=confidence,
                )
                entities.append(entity)
                detected_values.add(value)

        return entities

    def _is_valid_name_format(self, value: str) -> bool:
        """
        Validate if a value follows name format.

        Args:
            value: Value to validate

        Returns:
            True if valid name format, False otherwise
        """
        return bool(
            re.match(r'^[A-Z][a-z]*(?:\s+[A-Z][a-z]*)*$', value) or
            re.match(r'^[A-Z]+(?:\s+[A-Z]+)*$', value)
        )

    def _is_all_caps(self, value: str) -> bool:
        """
        Check if a value is in all caps.

        Args:
            value: Value to check

        Returns:
            True if all caps, False otherwise
        """
        return bool(re.match(r'^[A-Z]+(?:\s+[A-Z]+)*$', value))

    def _has_context_keyword(self, text: str, keywords: set) -> bool:
        """
        Check if text contains any of the given keywords.

        Args:
            text: Text to search
            keywords: Set of keywords to look for

        Returns:
            True if any keyword is found, False otherwise
        """
        text_lower = text.lower()
        return any(kw.lower() in text_lower for kw in keywords)