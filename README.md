# HBP100 v2.0.0

**Contextual privacy firewall for intelligent PII masking**

HBP100 is a lightweight privacy layer that detects sensitive information, uses a machine learning policy engine to make contextual masking decisions, replaces sensitive values with placeholders, and restores them after external processing.

Designed to be simple, fast, and reusable.

---

## Philosophy

> Sensitive information should never reach external AI systems unnecessarily.

HBP100 masks sensitive entities locally, allows external processing using placeholders, and restores the original values afterward.

The library is designed to work with any LLM, OCR pipeline, or external service.

---

## Features

* Context-aware PII masking
* TF-IDF + LightGBM privacy engine
* Placeholder generation and validation
* Metadata vault for reversible masking
* Restore API
* Modular architecture
* Lightweight package (~590 KB wheel)
* Sub-millisecond average latency
* Open source (MIT License)
* Works with any LLM

---

## Performance

| Metric          | Value                                |
| --------------- | ------------------------------------ |
| Average latency | ~0.136 ms/sample                     |
| Accuracy        | ~91%                                 |
| F1 Score        | ~0.91                                |
| Package size    | ~590 KB                              |
| Architecture    | Hybrid ML + deterministic extractors |

---

## Supported Entities

* Names
* Email addresses
* Phone numbers
* MRNs
* Policy numbers
* Case IDs
* Dates
* Addresses
* Hospitals
* Passport numbers

---

## Installation

```bash
pip install hbp100
```

---

## Quick Start

```python
from hbp100 import mask, restore

text = """
Patient John Doe, phone 9876543210.
Prescribed Metformin 500mg daily.
"""

# Mask sensitive values
masked = mask(text)

print(masked)

# Patient [NAME_1], phone [PHONE_1].
# Prescribed Metformin 500mg daily.

# Send to any external processor or LLM
response = external_llm(masked)

# Restore original values
final = restore(response)

print(final)

# Patient John Doe, phone 9876543210.
# Prescribed Metformin 500mg daily.
```

---

## Metadata Vault

Mappings are stored locally and can be inspected:

```python
from hbp100 import metadata_vault

metadata_vault.show()
```

Example:

```python
{
    "[NAME_1]": "John Doe",
    "[PHONE_1]": "9876543210"
}
```

Clear mappings:

```python
metadata_vault.clear()
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
ML Policy Engine
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

---

### Restore placeholders

```python
from hbp100 import restore

restored = restore(response)
```

---

### Metadata Vault

```python
from hbp100 import metadata_vault

metadata_vault.show()
metadata_vault.get()
metadata_vault.clear()
```

---

### Full Engine

```python
from hbp100 import HBP100

engine = HBP100()

result = engine.process(text)
```

---

## Architecture

```
hbp100/
├── core/
├── extractors/
├── policy_engine/
├── placeholders/
├── schemas/
├── assets/
├── tests/
└── utils/
```

---

## Why HBP100?

Traditional masking systems rely entirely on hardcoded rules.

HBP100 combines deterministic entity extraction with a machine learning policy engine to make contextual privacy decisions while remaining lightweight and fast.

By using reversible placeholders and a metadata vault, HBP100 enables secure external processing without losing information.

---

## Applications

* Healthcare document processing
* Insurance workflows
* OCR pipelines
* AI assistants
* Customer support systems
* Privacy-preserving LLM workflows
* Data preprocessing pipelines

---

## Limitations

HBP100 is a lightweight privacy framework and does not guarantee perfect extraction accuracy.

Entity recognition depends on extractor coverage and the machine learning policy engine. Some edge cases may not always be detected.

The library is designed for privacy-preserving workflows and should not be considered a substitute for specialized compliance or security systems.

---

## Built With

* Python
* scikit-learn
* LightGBM
* joblib

---

## License

MIT License

---

## Repository

https://github.com/Erox-02/humming-bird-v2

---

## Author

**Dipanjan Dutta**

---

## Version

Current release:

**HBP100 v2.0.0**
