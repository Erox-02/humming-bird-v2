import re
from typing import Dict, Optional, List

from hbp100.placeholders.metadata_vault import metadata_vault
from hbp100.utils.logger import get_logger

logger = get_logger(**name**)

def restore(text: str) -> str:
"""
Simple public API:

```
    restored = restore(response)
"""

restored = text

for placeholder, value in metadata_vault.get().items():
    restored = restored.replace(placeholder, value)

return restored
```

class PlaceholderRestorer:
"""
Advanced placeholder restoration.
"""

```
def __init__(self, metadata: Optional[Dict[str, str]] = None):
    self.metadata = metadata or {}
    self._pattern = re.compile(r"\[[A-Z_]+_\d+\]")

def restore(self, text: str, metadata: Optional[Dict[str, str]] = None) -> str:
    if metadata is not None:
        self.metadata = metadata

    restored = text

    for placeholder in sorted(self.metadata.keys(), key=len, reverse=True):
        restored = restored.replace(
            placeholder,
            self.metadata[placeholder]
        )

    return restored

def reset(self):
    self.metadata.clear()

def get_remaining_placeholders(self, text: str) -> List[str]:
    return self._pattern.findall(text)

def has_placeholders(self, text: str) -> bool:
    return bool(self._pattern.search(text))
```
