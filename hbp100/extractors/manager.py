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
    def __init__(self, extractors: Optional[List[BaseExtractor]] = None):
        self.extractors = extractors or self._default_extractors()
        self._type_map = self._build_type_map()
        logger.info(f"Extractor manager initialized with {len(self.extractors)} extractors")

    def _default_extractors(self) -> List[BaseExtractor]:
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
        type_map = {}
        for extractor in self.extractors:
            for entity_type in extractor.supported_types:
                type_map[entity_type] = extractor
        return type_map

    def extract_all(self, text: str) -> List[Entity]:
        all_entities = []
        detected_values = set()

        for extractor in self.extractors:
            try:
                entities = extractor.extract(text)
                for entity in entities:
                    if entity.value not in detected_values:
                        all_entities.append(entity)
                        detected_values.add(entity.value)
            except Exception as e:
                logger.error(f"Extractor {extractor.__class__.__name__} failed: {e}")

        all_entities.sort(key=lambda e: (e.start, -(e.end - e.start)))

        filtered = []
        for entity in all_entities:
            overlap = False
            for kept in filtered:
                if not (entity.end <= kept.start or entity.start >= kept.end):
                    overlap = True
                    break
            if not overlap:
                filtered.append(entity)

        return filtered

    def extract_by_type(self, text: str, entity_type: EntityType) -> List[Entity]:
        extractor = self._type_map.get(entity_type)
        if not extractor:
            logger.warning(f"No extractor found for entity type: {entity_type}")
            return []

        try:
            entities = extractor.extract(text)
            return [e for e in entities if e.type == entity_type]
        except Exception as e:
            logger.error(f"Extractor for {entity_type} failed: {e}")
            return []

    def register(self, extractor: BaseExtractor):
        self.extractors.append(extractor)
        for entity_type in extractor.supported_types:
            self._type_map[entity_type] = extractor
        logger.info(f"Registered extractor: {extractor.__class__.__name__}")

    def unregister(self, extractor_class: Type[BaseExtractor]):
        self.extractors = [e for e in self.extractors if not isinstance(e, extractor_class)]
        self._type_map = self._build_type_map()
        logger.info(f"Unregistered extractors of type: {extractor_class.__name__}")
