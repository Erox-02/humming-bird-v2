# HBP100 v3.0.0

**Native Rust contextual privacy firewall for intelligent PII masking**

HBP100 is a lightweight privacy layer that detects sensitive information locally, uses a contextual machine learning policy engine to decide whether detected entities should be masked, replaces sensitive values with placeholders, and restores them after external processing.

Hbp100 is designed to work with llms, ocr pipelines, apis, and other external processing systems without unnecessarily exposing sensitive information.

---

## Philosophy

> Sensitive information should never reach an external system unnecessarily.

hbp100 performs entity detection and privacy decisions locally.

Sensitive values are replaced with placeholders before text is sent to an external llm.the original values remain inside the local metadata vault and can be restored afterward.

---

## Architecture

Hbp100 separates *entity extraction* from *privacy policy decisions*.

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
````

The extractor layer determines *what an entity is*.

The machine learning policy engine determines *whether that entity should be masked in its context*.

This separation allows hbp100 to remain modular but also making contextual privacy decisions .

---

## What's New in v3.0.0

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
Rust (rust is better far better)
 ├── Extractors
 ├── Feature extraction
 ├── LightGBM inference
 ├── Policy engine
 └── Placeholder system
```

### Native LightGBM Policy Engine

The v3 policy engine uses LightGBM through the Rust "lightgbm3" crate.

The trained model's path is 

```
assets/hbp100-v3.lgb
```

the same model artifact can be loaded directly by the Rust runtime.

### Context-Aware Decisions

The complete surrounding sentence/document context is retained when generating the ml feature vector.

This allows decisions to depend on how an entity is being used rather than treating every entity value independently.

For example:

```
"My birthday is 12/03/2010."

"My MRN is 12/03/2010."
```

The same-looking value can have completely different privacy implications depending on context.

---

## Machine learning

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

The current training dataset contains:


The samples are split by document rather than randomly splitting individual entities from the same document.
This prevents entities from the same document from leaking between training and evaluation sets.

### Feature Space

The current v3 feature extractor produces:

for each entity/context sample.

### Eval

Current held-out evaluation results:

| Metric    |     Result |
| --------- | ---------: |
| Accuracy  | **91.34%** |
| Precision | **95.57%** |
| Recall    | **84.59%** |
| F1 Score  | **89.75%** |

THe given benchmarks are from the test dataset .(its currently lower than python one)


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

Hbp100 currently includes modular extractors for entities including:

- Names
- Email addresses
- Phone numbers
- Dates
- Addresses
- IDs
- Medical information
- Other configurable identifier types

The extractor architecture is modular, allowing additional detectors to be added independently of the ml policy engine.

---

## Rust API

Hbp100 exposes a simple Rust API through the "HBP100" type.

### Create an engine

```
use hbp100::HBP100;

let mut engine = HBP100::new();
```

### Process text

```
let result = engine.process(
    "Patient John Doe, phone 9876543210.",
    None,
);

println!("{}", result.masked_text);
```

The result contains:

```
result.original_text
result.masked_text
result.entities
result.decisions
result.metadata
result.has_pii
```

### Intent-aware processing

An optional intent can be supplied to the policy engine:

```
let result = engine.process(
    text,
    Some("hospital_discharge"),
);
```

The intent becomes part of the contextual decision process.
---

## Restoration

Masked values are stored locally and can be restored after external processing.
```
let restored = engine.restore(&response);
```
Metadata can also be supplied explicitly:

```
let restored = engine.restore_with_metadata(
    &response,
    metadata,
);
```
The metadata vault remains local to the hbp100 runtime.

---

## Response Validation

Hbp100 can validate returned text before restoration:

```
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

Hbp100 follows a simple principle:

```text
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

The external system should only receive the sanitized representation.

Hbp100 does not require an external AI model to make privacy decisions during inference.

---

## Project Structure

.
├── assets
│   └── hbp100-v3.lgb
├── Cargo.lock
├── Cargo.toml
├── dataset.json
├── LICENSE
├── README.md
└── src
    ├── api.rs
    ├── core
    │   ├── engine.rs
    │   ├── metadata.rs
    │   ├── mod.rs
    │   └── pipeline.rs
    ├── extractors
    │   ├── addresses.rs
    │   ├── base.rs
    │   ├── dates.rs
    │   ├── emails.rs
    │   ├── ids.rs
    │   ├── manager.rs
    │   ├── medical.rs
    │   ├── mod.rs
    │   ├── names.rs
    │   └── phones.rs
    ├── interfaces
    │   ├── extractor.rs
    │   ├── mod.rs
    │   ├── placeholder.rs
    │   └── predictor.rs
    ├── lib.rs
    ├── ml
    │   ├── dataset.rs
    │   ├── features.rs
    │   ├── model.rs
    │   └── mod.rs
    ├── placeholders
    │   ├── generator.rs
    │   ├── metadata.rs
    │   ├── mod.rs
    │   ├── restore.rs
    │   └── validator.rs
    ├── policy_engine
    │   ├── context_builder.rs
    │   ├── mod.rs
    │   └── predictor.rs
    ├── schemas
    │   ├── decision.rs
    │   ├── entity.rs
    │   ├── mod.rs
    │   ├── placeholder.rs
    │   └── result.rs
    └── utils
        ├── helpers.rs
        ├── logger.rs
        └── mod.rs

11 directories, 46 files
(lol i used tree XD)

---

## Model Development

The model can be trained directly from Rust:

```
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

This verifies that the saved LightGBM artifact can be loaded and executed directly by rust.

---

## Installation

Hbp100 is a published rust crate so 
```
cargo install hbp100
``` 
does the work

or

### Clone the repository:

```
git clone <repository>
cd humming-bird-v3
```

### Build:

```
cargo build
```

---

## Example

```
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

Hbp100 intentionally combines deterministic extraction with light gbm model.

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

The extractor does not need to decide whether an entity is sensitive enough to expose.
The policy engine makes that decision using the complete contextual feature representation.

---

## Applications

Hbp100 can be used as a local privacy layer for:

- healthcare document processing
- insurance workflows
- ocr pipelines
- ai assistants
- llm applications
- customer support systems
- data preprocessing
- privacy-preserving API workflows
- external ai processing pipelines

---

## Performance

Hbp100 is designed as a lightweight local inference system.

The v3 runtime uses:

* Native Rust execution
* Native LightGBM inference
* Modular deterministic extractors
* Local feature extraction
* Reversible placeholder masking
* No Python runtime dependency

Model quality is currently:

```text
Accuracy   91.34%
Precision  95.57%
Recall     84.59%
F1         89.75%
```

Performance should be benchmarked on the target hardware and workload rather than assuming a fixed latency across systems.

---

## Limitations

Hbp100 does not guarantee perfect entity detection or privacy protection.

Its effectiveness depends on:

* Extractor coverage
* Feature quality
* Training data quality
* Model performance
* Input distribution
* Correct handling of placeholders by downstream systems

Hbp100 is a privacy-preserving framework and should not be considered a complete compliance, security, or data-loss-prevention solution by itself.

Applications handling sensitive information should use appropriate security controls and validation around Hbp100.

---

## Technology

Hbp100 v3 is built with:

~ Rust
~ lightgbm3
~ Serde
~ Regex
~ Chrono
~ Modular Rust extractors

---

## Roadmap

Potential future work includes:

- Additional entity extractors
- Improved contextual features
- Larger and more diverse training datasets
- Session-aware metadata handling
- Additional model evaluation
- Performance benchmarking
- Further runtime optimization
---

## License

MIT License
---

## Author

*Dipanjan Dutta*
---

## Version
**HBP100 v3.0.0**
```