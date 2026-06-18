from typing import Dict

class MetadataVault:
"""
Stores placeholder ↔ original value mappings.
"""

```
def __init__(self):
    self._vault: Dict[str, str] = {}

def add(self, placeholder: str, original_value: str) -> None:
    self._vault[placeholder] = original_value

def get(self) -> Dict[str, str]:
    return self._vault.copy()

def clear(self) -> None:
    self._vault.clear()

def show(self) -> None:
    if not self._vault:
        print("Metadata vault is empty.")
        return

    for placeholder, value in self._vault.items():
        print(f"{placeholder} = {value}")
```

metadata_vault = MetadataVault()
