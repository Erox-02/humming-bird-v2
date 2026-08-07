# HBP100 v2.1.0

**Contextual privacy firewall for intelligent PII masking**

HBP100 is a lightweight privacy layer that detects sensitive information, uses a machine learning policy engine to make contextual masking decisions, replaces sensitive values with placeholders, and restores them after external processing.

Designed to be simple, fast, and reusable.

---

## Philosophy

> Sensitive information should never reach external AI systems unnecessarily.

HBP100 masks sensitive entities locally, allows external processing using placeholders, and restores the original values afterward.

The library is designed to work with any LLM, OCR pipeline, or external service.

---

## Design

Unlike traditional PII masking systems, HBP100 separates entity extraction from privacy policy decisions.

```
Entity Detection
        +
Context-Aware ML Policy
        =
Intelligent Privacy Decisions
```

This architecture allows HBP100 to remain lightweight while making contextual masking decisions without requiring large language models during inference.

---

## Features

- Context-aware PII masking
- Hybrid deterministic + Machine Learning architecture
- TF-IDF + LightGBM privacy policy engine
- Intelligent MASK / KEEP decision engine
- Placeholder generation and validation
- Metadata vault for reversible masking
- Restore API
- Modular architecture
- Lightweight package (~445 KB wheel)
- Sub-millisecond average latency
- Open source (MIT License)
- Works with any LLM

---

## What's New in v2.1

### Improved ML Pipeline

- Retrained LightGBM privacy policy model
- Expanded contextual training dataset
- Improved MASK / KEEP decision quality
- Reduced false positives
- Better contextual reasoning

### Training Pipeline

- Dataset validation
- Duplicate entity detection
- Context consistency checking
- Cross-validation support
- Automatic metric reporting
- Serialized model artifacts
- Reproducible training pipeline

### Validation

- Malformed JSON detection
- Missing field validation
- Invalid decision detection
- Entity consistency checking
- Case sensitivity warnings
- Duplicate entity detection

---

## Performance

| Metric | Value |
|---------|-------|
| Average latency | ~0.136 ms/sample |
| Accuracy | **93.93%** |
| F1 Score | **92.55%** |
| Cross Validation F1 | **92.25% ± 0.54%** |
| Package size | ~445 KB wheel |
| Architecture | Hybrid ML + deterministic extractors |

---

## ML Pipeline

```
Dataset
    │
    ▼
Dataset Validation
    │
    ▼
Flatten Context Samples
    │
    ▼
TF-IDF Vectorization
    │
    ▼
LightGBM Policy Engine
    │
    ▼
privacy_model.pkl
```

The policy engine predicts whether each detected entity should be **MASK** or **KEEP** based on contextual information.

---

## Runtime Pipeline

```
Input Text
      │
      ▼
Entity Extractors
      │
      ▼
Context Builder
      │
      ▼
LightGBM Policy Engine
      │
      ▼
Placeholder Generator
      │
      ▼
Metadata Vault
      │
      ▼
Masked Text
      │
      ▼
External LLM
      │
      ▼
restore()
      │
      ▼
Original Text
```

---

## Supported Entities

- Names
- Email addresses
- Phone numbers
- MRNs
- Policy numbers
- Case IDs
- Dates
- Addresses
- Hospitals
- Passport numbers

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

### Metadata Vault

```python
from hbp100 import metadata_vault

metadata_vault.show()
metadata_vault.get()
metadata_vault.clear()
```

### Full Engine

```python
from hbp100 import HBP100

engine = HBP100()
result = engine.process(text)
```

---

## Why HBP100?

HBP100 separates **entity extraction** from **privacy policy decisions**.

Deterministic extractors identify candidate entities, while a LightGBM-based contextual policy engine determines whether each entity should be masked or preserved.

This hybrid architecture provides predictable extraction with intelligent context-aware privacy decisions while remaining lightweight and extremely fast.

---

## Applications

- Healthcare document processing
- Insurance workflows
- OCR pipelines
- AI assistants
- Customer support systems
- Privacy-preserving LLM workflows
- Data preprocessing pipelines

---

## Built With

- Python
- LightGBM
- scikit-learn
- joblib
- NumPy
- TF-IDF Vectorizer

---

## Roadmap

- Native C extractors for higher throughput
- Session-aware metadata vault
- Additional entity extractors
- ONNX model export
- Performance benchmarking suite

---

## Limitations

HBP100 is a lightweight privacy framework and does not guarantee perfect extraction accuracy.

Entity recognition depends on extractor coverage and the machine learning policy engine. Some edge cases may not always be detected.

The library is designed for privacy-preserving workflows and should not be considered a substitute for specialized compliance or security systems.

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

Current Release

**HBP100 v2.1.0**
```
