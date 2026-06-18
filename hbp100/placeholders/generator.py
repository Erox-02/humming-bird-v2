from collections import defaultdict
from typing import Dict, Optional
import re

from hbp100.schemas.entity import Entity
from hbp100.schemas.placeholder import Placeholder
from hbp100.utils.logger import get_logger
from hbp100.placeholders.metadata_vault import metadata_vault

logger = get_logger(**name**)

class PlaceholderGenerator:
"""
Generates placeholders and stores mappings in metadata_vault.
"""

```
PLACEHOLDER_PATTERN = re.compile(r"\[([A-Z_]+)_(\d+)\]")

def __init__(self):
    self._counters: Dict[str, int] = defaultdict(int)
    self._placeholders: Dict[str, Placeholder] = {}

def generate(self, entity: Entity) -> str:
    entity_type = entity.type.value

    self._counters[entity_type] += 1
    count = self._counters[entity_type]

    placeholder = f"[{entity_type}_{count}]"

    obj = Placeholder(
        placeholder=placeholder,
        original_value=entity.value,
        entity_type=entity_type,
    )

    self._placeholders[placeholder] = obj
    entity.placeholder = placeholder

    metadata_vault.add(placeholder, entity.value)

    logger.debug(f"Generated {placeholder}")

    return placeholder

def get_metadata(self) -> Dict[str, str]:
    return metadata_vault.get()

def get_original_value(self, placeholder: str) -> Optional[str]:
    return metadata_vault.get().get(placeholder)

def reset(self):
    self._counters.clear()
    self._placeholders.clear()
    metadata_vault.clear()

@staticmethod
def is_valid_placeholder(text: str) -> bool:
    return bool(PlaceholderGenerator.PLACEHOLDER_PATTERN.fullmatch(text))
```
