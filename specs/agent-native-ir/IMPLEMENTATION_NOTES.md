# Implementation Notes – Agent-Native Semantic Representation

## Current state
Design complete. Minimal types added to prime-core.

## Next implementation steps

### 1. Semantic metadata extraction
Extend `prime-parser::Extractor` to derive:
- effects: from function body analysis (IO calls, file ops)
- inputs/outputs: from signature parsing
- capabilities: from imports/uses
- evidence: source location

Start with Rust functions: detect `std::fs`, `std::io`, `println!` → IO effect.

### 2. KnowledgeGraph extension
Add `semantic_index: HashMap<EntityId, SemanticMetadata>` to `KnowledgeGraph` or store as parallel map.

### 3. Serialization
Implement `prime-index::semantic_serialization`:
- JSON for debugging
- Line-oriented agent view
- Binary FlatBuffers for storage

### 4. Query layer
Extend `prime-core::ToolIntent` with Semantic variants.
Update `prime-index::ToolExecutor::execute` to handle semantic queries.

### 5. Experiment harness
Create benchmark comparing:
A baseline
B AST
C semantic IR

Collect tokens per fact, accuracy, latency.

## Files to modify
- prime-rs/prime-core/src/types.rs – add semantic field to Entity optional
- prime-rs/prime-parser/src/extractor.rs – effect inference
- prime-rs/prime-index/src/serialize.rs – semantic serialization
- prime-rs/prime-query/src/agent.rs – semantic query handling

## Risks
- Over-extraction of unsupported semantics
- Token bloat if serialization not compact
- Performance regression

Mitigate with evidence-backed facts only and token budget enforcement.
