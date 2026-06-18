# hbp100

**A lightweight contextual privacy firewall for intelligent PII masking.**

`hbp100` detects sensitive entities, uses a machine learning policy engine to decide whether they should be hidden, replaces them with placeholders, and later restores them after external processing.

Designed to be simple, fast, and reusable.

---

## Features

~  Intelligent PII masking
~  TF-IDF + LightGBM policy engine
~  Fast inference
~  Lightweight package
~  Placeholder restoration
~  Metadata vault
~  Modular architecture
~  No hardcoded privacy rules
~  Works with any LLM
~  Supports medical, financial, and general text

---

## Installation

```bash
pip install hbp100
```

---

## Quick Start

```python
from hbp100 import mask, restore, metadata_vault

text = """
Patient John Doe, phone 9876543210.
Prescribed Metformin 500mg daily.
"""

# Mask sensitive values
masked = mask(text)

print(masked)
# Patient [NAME_1], phone [PHONE_1].
# Prescribed Metformin 500mg daily.

# Send to any LLM
response = llm(masked)

# Restore placeholders
final = restore(response)

print(final)

# Inspect mappings
metadata_vault.show()
```

Output:

```
[NAME_1] = John Doe
[PHONE_1] = 9876543210
```

---

## Advanced Usage

```python
from hbp100 import HBP100

engine = HBP100()

result = engine.process(text)

print(result.masked_text)
print(result.entities)
print(result.decisions)
print(result.metadata)
```

---

## Example Workflow

```
Text
 ↓
Entity Extractors
 ↓
Policy Engine
 ↓
Placeholder Generator
 ↓
Metadata Vault
 ↓
Masked Text
 ↓
External Processing
 ↓
restore()
 ↓
Original Values Restored
```

---

## Public API

### Mask text

```python
from hbp100 import mask

masked = mask(text)
```

### Restore placeholders

```python
from hbp100 import restore

restored = restore(response)
```

### View metadata

```python
from hbp100 import metadata_vault

metadata_vault.show()

metadata_vault.get()

metadata_vault.clear()
```

### Full engine

```python
from hbp100 import HBP100

engine = HBP100()

result = engine.process(text)
```

---

## Architecture

```
hbp100/
├── extractors/
├── policy_engine/
├── placeholders/
├── schemas/
├── core/
├── assets/
└── tests/
```

---

## Built With

~ Python
~ scikit-learn
~ LightGBM
~ joblib

---

## License

MIT License

---

## Repository

https://github.com/Erox-02/humming-bird-v2

---

## Author

**Dipanjan Dutta**
