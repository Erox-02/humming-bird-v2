from typing import List, Dict, Type, Optional
import logging

from hbp100.extractors.base import BaseExtractor
from hbp100.extractors.names import NameExtractor
from hbp100.extractors.phones import PhoneExtractor
from hbp100.extractors.emails import EmailExtractor
from hbp100.extractors.dates import DateExtractor
from hbp100.extractors.addresses import AddressExtractor
from hbp100.extractors.ids import IDExtractor
from hbp100.extractors.medical import HospitalExtractor
from hbp100.schemas.entity import Entity, EntityType

logger = logging.getLogger(__name__)


class ExtractorManager:
    """
    Manages and coordinates multiple entity extractors.

    Provides a unified interface for entity extraction across all extractor types.
    """

    def __init__(self, extractors: Optional[List[BaseExtractor]] = None):
        """
        Initialize the extractor manager.

        Args:
            extractors: Optional list of extractors (uses defaults if None)
        """
        self.extractors = extractors or self._default_extractors()
        self._type_map = self._build_type_map()
        logger.info(f"Extractor manager initialized with {len(self.extractors)} extractors")

    def _default_extractors(self) -> List[BaseExtractor]:
        """Create default extractors."""
        return [
            NameExtractor(),
            PhoneExtractor(),
            EmailExtractor(),
            DateExtractor(),
            AddressExtractor(),
            IDExtractor(),
            HospitalExtractor(),
        ]

    def _build_type_map(self) -> Dict[EntityType, BaseExtractor]:
        """Build mapping from entity type to extractor."""
        type_map = {}
        for extractor in self.extractors:
            for entity_type in extractor.supported_types:
                type_map[entity_type] = extractor
        return type_map

    def extract_all(self, text: str) -> List[Entity]:
        """
        Extract all entities from text using all registered extractors.

        Args:
            text: Input text to analyze

        Returns:
            List of Entity objects sorted by start position
        """
        all_entities = []
        detected_values = set()

        for extractor in self.extractors:
            try:
                entities = extractor.extract(text)
                # Deduplicate by value
                for entity in entities:
                    if entity.value not in detected_values:
                        all_entities.append(entity)
                        detected_values.add(entity.value)
            except Exception as e:
                logger.error(f"Extractor {extractor.__class__.__name__} failed: {e}")

        # Sort by start position for consistent processing
        all_entities.sort(key=lambda e: e.start)

        return all_entities

    def extract_by_type(self, text: str, entity_type: EntityType) -> List[Entity]:
        """
        Extract entities of a specific type.

        Args:
            text: Input text to analyze
            entity_type: Entity type to extract

        Returns:
            List of Entity objects of the specified type
        """
        extractor = self._type_map.get(entity_type)
        if not extractor:
            logger.warning(f"No extractor found for entity type: {entity_type}")
            return []

        try:
            entities = extractor.extract(text)
            # Filter by type
            return [e for e in entities if e.type == entity_type]
        except Exception as e:
            logger.error(f"Extractor for {entity_type} failed: {e}")
            return []

    def register(self, extractor: BaseExtractor):
        """
        Register a new extractor.

        Args:
            extractor: Extractor to register
        """
        self.extractors.append(extractor)
        # Update type map
        for entity_type in extractor.supported_types:
            self._type_map[entity_type] = extractor
        logger.info(f"Registered extractor: {extractor.__class__.__name__}")

    def unregister(self, extractor_class: Type[BaseExtractor]):
        """
        Unregister all extractors of a given class.

        Args:
            extractor_class: Class of extractors to remove
        """
        self.extractors = [e for e in self.extractors if not isinstance(e, extractor_class)]
        # Rebuild type map
        self._type_map = self._build_type_map()
        logger.info(f"Unregistered extractors of type: {extractor_class.__name__}")