import re
from abc import ABC, abstractmethod
from typing import List, Set, Optional, Pattern

from hbp100.schemas.entity import Entity, EntityType
from hbp100.utils.logger import get_logger

logger = get_logger(__name__)


class BaseExtractor(ABC):
    def __init__(self):
        self._patterns: List[Pattern] = []
        self._compile_patterns()

    @abstractmethod
    def _compile_patterns(self):
        pass

    @abstractmethod
    def extract(self, text: str) -> List[Entity]:
        pass

    @property
    @abstractmethod
    def supported_types(self) -> List[EntityType]:
        pass

    def supports(self, entity_type: EntityType) -> bool:
        return entity_type in self.supported_types

    def _validate_text(self, text: str) -> None:
        if not text or not text.strip():
            raise ValueError("Input text cannot be empty")

    def _extract_matches(
        self,
        text: str,
        patterns: List[Pattern],
        entity_type: EntityType,
        confidence: float = 0.85,
    ) -> List[Entity]:
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
        return bool(
            re.match(r'^[A-Z][a-z]*(?:\s+[A-Z][a-z]*)*$', value) or
            re.match(r'^[A-Z]+(?:\s+[A-Z]+)*$', value)
        )

    def _is_all_caps(self, value: str) -> bool:
        return bool(re.match(r'^[A-Z]+(?:\s+[A-Z]+)*$', value))

    def _has_context_keyword(self, text: str, keywords: set) -> bool:
        text_lower = text.lower()
        return any(kw.lower() in text_lower for kw in keywords)
