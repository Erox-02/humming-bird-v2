Here's the fixed and cleaned-up README:

---

# HBP100 v3.1.0

**Native Rust contextual privacy firewall for intelligent PII masking**

HBP100 is a lightweight privacy layer that detects sensitive information locally, uses a contextual machine learning policy engine to decide whether detected entities should be masked, replaces sensitive values with placeholders, and restores them after external processing.

HBP100 is designed to work with LLMs, OCR pipelines, APIs, and other external processing systems without unnecessarily exposing sensitive information.

---

## Philosophy

> Sensitive information should never reach an external system unnecessarily.

HBP100 performs entity detection and privacy decisions locally. Sensitive values are replaced with placeholders before text is sent to an external LLM. The original values remain inside the local metadata vault and can be restored afterward.

---

## Architecture

HBP100 separates *entity extraction* from *privacy policy decisions*.

```
                Input Text
                    │
                    ▼
          ┌──────────────────┐
          │ Entity Extractors│
          └────────┬─────────┘
                   │
                   ▼
             Detected Entities
                   │
                   ▼
          ┌──────────────────┐
          │ Contextual Policy│
          │     Engine       │
          └────────┬─────────┘
                   │
                   ▼
              MASK / KEEP
                   │
          ┌────────┴─────────┐
          │                  │
        MASK                KEEP
          │                  │
          ▼                  │
   Placeholder Generator     │
          │                  │
          └────────┬─────────┘
                   ▼
              Masked Text
```

The extractor layer determines *what an entity is*. The machine learning policy engine determines *whether that entity should be masked in its context*. This separation allows HBP100 to remain modular while making contextual privacy decisions.

---

## Release History

### v3.1.0 — Sessions + Python API

v3.1.0 adds stateful session support for multi-step LLM workflows while keeping the existing stateless API unchanged.

- Persistent `Session` state for placeholder mappings
- `SessionManager` for in-memory session lifecycle management
- Session-aware placeholder generation
- Per-entity counters continue across calls in the same session
- `restore_with_session()` restores using accumulated session metadata
- Session IDs are SHA256-based
- Sessions are currently stored in memory
- Stateless `process()` remains unchanged for backward compatibility
- Python bindings and Python API are part of the supported release surface
- Rust crate and Python package are both supported

### v3.0.1 — Bug Fixes + Performance

v3.0.1 focused on extractor reliability and runtime performance.

- Extractor regex fixes
- Performance improvements
- Native Rust package/runtime continued as the core execution path
- PyO3-based Python bindings and the Python API were introduced alongside the Rust core

---

## What's New in v3.1.0

### Native Rust Runtime

v3 removes the previous Python ML runtime and moves the production inference stack entirely into Rust.

```
v2
Python
 └── scikit-learn / LightGBM
       ↓
    Python bridge
       ↓
      Rust

v3
Rust
 ├── Extractors
 ├── Feature extraction
 ├── LightGBM inference
 ├── Policy engine
 └── Placeholder system
```

### Native LightGBM Policy Engine

The v3 policy engine uses LightGBM through the Rust `lightgbm3` crate. The trained model's path is:

```
assets/hbp100-v3.lgb
```

The same model artifact can be loaded directly by the Rust runtime.

### Context-Aware Decisions

The complete surrounding sentence/document context is retained when generating the ML feature vector. This allows decisions to depend on how an entity is being used rather than treating every entity value independently.

For example:
```
"My birthday is 12/03/2010."
"My MRN is 12/03/2010."
```

The same-looking value can have completely different privacy implications depending on context.

---

## Machine Learning

The v3 training pipeline is implemented in Rust.

```
dataset.json
     │
     ▼
Dataset Loader
     │
     ▼
Training Samples
     │
     ▼
Feature Extraction
     │
     ▼
LightGBM
     │
     ▼
hbp100-v3.lgb
```

The dataset contains complete input text, intent, detected entity type, entity value, and the expected "mask" / "keep" decision.

### Dataset

The samples are split by document rather than randomly splitting individual entities from the same document. This prevents entities from the same document from leaking between training and evaluation sets.

### Feature Space

The current v3 feature extractor produces features for each entity/context sample.

### Evaluation

Current held-out evaluation results:

| Metric    | Result |
| --------- | ------:|
| Accuracy  | **91.34%** |
| Precision | **95.57%** |
| Recall    | **84.59%** |
| F1 Score  | **89.75%** |

The given benchmarks are from the test dataset (currently lower than the Python version).

---

## Runtime Pipeline

```
Input Text
    │
    ▼
Entity Extractors
    │
    ▼
Context + Entity Features
    │
    ▼
LightGBM Policy Engine
    │
    ▼
MASK / KEEP Decision
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
External LLM / API / Processor
    │
    ▼
restore()
    │
    ▼
Original Text
```

Everything required for production inference is executed locally.

---

## Supported Entities

HBP100 currently includes modular extractors for entities including:

- Names
- Email addresses
- Phone numbers
- Dates
- Addresses
- IDs
- Medical information
- Other configurable identifier types

The extractor architecture is modular, allowing additional detectors to be added independently of the ML policy engine.

---

## Python API

HBP100 provides a Python API backed by the same native Rust privacy core.

### Basic processing

```python
from hbp100 import HBP100

engine = HBP100()

result = engine.process(
    "Patient John Doe, MRN: 123456"
)

print(result["masked_text"])
```

Example output:
```
Patient [NAME_1], MRN: [MRN_1]
```

The Python result exposes the processed result as a Python mapping containing fields such as:
- `masked_text`
- `entities`
- `decisions`
- `has_pii`
- `metadata`

The Python layer is an API boundary; the extraction, contextual policy evaluation, and placeholder processing remain implemented in Rust.

---

## Rust API

HBP100 exposes a simple Rust API through the `HBP100` type.

### Create an engine

```rust
use hbp100::HBP100;

let mut engine = HBP100::new();
```

### Process text

```rust
let result = engine.process(
    "Patient John Doe, phone 9876543210.",
    None,
);

println!("{}", result.masked_text);
```

The result contains:
- `result.original_text`
- `result.masked_text`
- `result.entities`
- `result.decisions`
- `result.metadata`
- `result.has_pii`

### Intent-aware processing

An optional intent can be supplied to the policy engine:

```rust
let result = engine.process(
    text,
    Some("hospital_discharge"),
);
```

The intent becomes part of the contextual decision process.

---

## Session Management

HBP100 v3.1.0 adds sessions for workflows where multiple LLM interactions need to share the same placeholder state.

### Why sessions?

A stateless call can mask a piece of text and keep its metadata for that operation. LLM workflows often need something stronger: multiple requests in the same interaction must share the same placeholder mappings.

For example:
```
Request 1:
Patient John Doe, MRN: 123456
        ↓
Patient [NAME_1], MRN: [MRN_1]
```

The session retains:
```
[NAME_1] → John Doe
[MRN_1]  → 123456
```

A later call using the same session can append new mappings instead of starting from an empty placeholder state.

### SessionManager

`SessionManager` maintains persistent placeholder mappings across multiple LLM interactions.

When `process_with_session()` is called, HBP100 uses a session-aware placeholder generator. Entity counters continue within the session, producing placeholders such as:
```
[NAME_1]
[NAME_2]
[MRN_1]
[MRN_2]
```

New mappings are appended to the session metadata.

Conceptually:
```
Session
├── session ID
├── placeholder mappings
├── entity counters
├── created timestamp
└── updated timestamp
```

### Session processing

The session-aware API is intended for multi-step LLM workflows:

```
Input
  │
  ▼
SessionManager
  │
  ▼
process_with_session()
  │
  ├── extract entities
  ├── contextual policy decision
  ├── generate session-aware placeholders
  └── update session metadata
  │
  ▼
Masked text
  │
  ▼
External LLM / API
  │
  ▼
restore_with_session()
  │
  ▼
Restored response
```

A later call with the same session ID uses the accumulated metadata.

### Session storage

Sessions are currently stored **in memory** using a `HashMap`. Session IDs are generated using SHA256. The current implementation is intentionally memory-only. Persistent disk-backed sessions are planned for a future version.

### Stateless API compatibility

The existing stateless API remains unchanged:
```rust
engine.process(...)
```

Applications that do not need multi-request state can continue using the original API without sessions.

### Python session workflow

The same session concept is exposed through the Python API:

```python
from hbp100 import HBP100

engine = HBP100()

session_id = engine.create_session()

result = engine.process_with_session(
    session_id,
    "Patient John Doe, MRN: 123456"
)

print(result["masked_text"])
```

The exact Python method signature follows the released binding API. The important property is that subsequent `process_with_session()` calls using the same session ID reuse the accumulated placeholder metadata.

---

## Restoration

Masked values are stored locally and can be restored after external processing.

```rust
let restored = engine.restore(&response);
```

Metadata can also be supplied explicitly:

```rust
let restored = engine.restore_with_metadata(
    &response,
    metadata,
);
```

The metadata vault remains local to the HBP100 runtime.

---

## Response Validation

HBP100 can validate returned text before restoration:

```rust
let (valid, error) = engine.validate_response(response);
```

This allows applications to detect invalid or unexpected placeholder manipulation before restoring sensitive values.

---

## Placeholder System

Detected entities selected for masking are replaced with generated placeholders.

For example:
```
Input:
Patient John Doe, phone 9876543210.

Masked:
Patient [NAME_1], phone [PHONE_1].
```

The original values remain in local metadata:
```
[NAME_1]  → John Doe
[PHONE_1] → 9876543210
```

The external processor only receives the masked representation.

---

## Security Model

HBP100 follows a simple principle:

```
Sensitive data
      │
      ▼
Local extraction
      │
      ▼
Local ML decision
      │
      ▼
Local masking
      │
      ▼
External system
```

The external system should only receive the sanitized representation. HBP100 does not require an external AI model to make privacy decisions during inference.

---

## Project Structure

```
.
├── assets
│   └── hbp100-v3.lgb
├── Cargo.lock
├── Cargo.toml
├── dataset.json
├── LICENSE
├── README.md
└── src
    ├── api.rs
    ├── core
    │   ├── engine.rs
    │   ├── metadata.rs
    │   ├── mod.rs
    │   └── pipeline.rs
    ├── extractors
    │   ├── addresses.rs
    │   ├── base.rs
    │   ├── dates.rs
    │   ├── emails.rs
    │   ├── ids.rs
    │   ├── manager.rs
    │   ├── medical.rs
    │   ├── mod.rs
    │   ├── names.rs
    │   └── phones.rs
    ├── interfaces
    │   ├── extractor.rs
    │   ├── mod.rs
    │   ├── placeholder.rs
    │   └── predictor.rs
    ├── lib.rs
    ├── ml
    │   ├── dataset.rs
    │   ├── features.rs
    │   ├── model.rs
    │   └── mod.rs
    ├── placeholders
    │   ├── generator.rs
    │   ├── metadata.rs
    │   ├── mod.rs
    │   ├── restore.rs
    │   └── validator.rs
    ├── policy_engine
    │   ├── context_builder.rs
    │   ├── mod.rs
    │   └── predictor.rs
    ├── schemas
    │   ├── decision.rs
    │   ├── entity.rs
    │   ├── mod.rs
    │   ├── placeholder.rs
    │   └── result.rs
    └── utils
        ├── helpers.rs
        ├── logger.rs
        └── mod.rs
```

11 directories, 46 files

---

## Model Development

The model can be trained directly from Rust:

```bash
cargo run --release --bin train
```

The trained model is stored to:

```
assets/hbp100-v3.lgb
```

The model can then be tested independently:

```bash
cargo run --release --bin test
```

This verifies that the saved LightGBM artifact can be loaded and executed directly by Rust.

---

## Installation

HBP100 is distributed as both a **Rust crate** and a **Python package**.

### Rust

Add HBP100 to a Rust project:

```toml
[dependencies]
hbp100 = "3.1.0"
```

Or install the published crate:

```bash
cargo install hbp100
```

### Python

The Python API is backed by the native Rust core through PyO3.

```python
from hbp100 import HBP100

engine = HBP100()

result = engine.process(
    "Patient John Doe, MRN: 123456"
)

print(result["masked_text"])
```

For development/builds from the repository:

```bash
git clone <repository>
cd humming-bird-v3
maturin develop --release
```

The Python extension should be built in **release mode** for representative performance measurements.

---

## Example

```rust
use hbp100::HBP100;

fn main() {
    let mut engine = HBP100::new();

    let text = "
        Patient John Doe.
        Email john@example.com.
        Prescribed Metformin 500mg daily.
    ";

    let result = engine.process(
        text,
        Some("hospital_discharge"),
    );

    println!("Masked:");
    println!("{}", result.masked_text);

    println!("\nDecisions:");
    for decision in &result.decisions {
        println!(
            "{:?} -> {:?} ({:.4})",
            decision.entity.entity_type,
            decision.decision,
            decision.confidence
        );
    }
}
```

---

## Hybrid Architecture

HBP100 intentionally combines deterministic extraction with a LightGBM model.

```
┌─────────────────────────────┐
│     Deterministic Layer     │
│                             │
│ Regex / structured          │
│ entity extraction           │
└──────────────┬──────────────┘
               │
               ▼
        Candidate Entities
               │
               ▼
┌─────────────────────────────┐
│      ML Policy Layer        │
│                             │
│ Contextual LightGBM model   │
└──────────────┬──────────────┘
               │
               ▼
          MASK / KEEP
```

The extractor does not need to decide whether an entity is sensitive enough to expose. The policy engine makes that decision using the complete contextual feature representation.

---

## Applications

HBP100 can be used as a local privacy layer for:

- Healthcare document processing
- Insurance workflows
- OCR pipelines
- AI assistants
- LLM applications
- Customer support systems
- Data preprocessing
- Privacy-preserving API workflows
- External AI processing pipelines

---

## Performance

HBP100 is designed as a lightweight local inference system. The runtime uses:

- Native Rust execution
- Native LightGBM inference
- Modular deterministic extractors
- Local feature extraction
- Reversible placeholder masking
- Optional Python bindings through PyO3

### Python API benchmark

A release-mode Python binding benchmark on an Intel Core i5-1135G7(undervolted) produced:

```
100,000 iterations
5.3999 seconds
0.0540 ms/text
18,519 texts/sec
```

Longer 1,000,000-iteration runs measured approximately:

```
0.063–0.070 ms/text
~14,000–16,000 texts/sec
```

These are single-process benchmark results and should not be treated as universal throughput guarantees. Performance depends on hardware, workload, Python/runtime overhead, CPU power limits, and execution environment.

Model quality is currently:

```
Accuracy   91.34%
Precision  95.57%
Recall     84.59%
F1         89.75%
```

---

## Limitations

HBP100 does not guarantee perfect entity detection or privacy protection. Its effectiveness depends on:

- Extractor coverage
- Feature quality
- Training data quality
- Model performance
- Input distribution
- Correct handling of placeholders by downstream systems

HBP100 is a privacy-preserving framework and should not be considered a complete compliance, security, or data-loss-prevention solution by itself. Applications handling sensitive information should use appropriate security controls and validation around HBP100.

---

## Technology

HBP100 v3 is built with:

- Rust
- lightgbm3
- Serde
- Regex
- Chrono
- Modular Rust extractors
- PyO3 Python bindings
- SHA256 session IDs

---

## Roadmap

Potential future work includes:

- Additional entity extractors
- Improved contextual features
- Larger and more diverse training datasets
- Persistent disk-backed sessions
- Additional model evaluation
- Further runtime optimization
- More LLM/provider integrations
- Batch and concurrent processing

Session-aware metadata handling is now implemented in v3.1.0.

---

## License

MIT License

---

## Author

*Dipanjan Dutta*

---

## Version

**HBP100 v3.1.0**