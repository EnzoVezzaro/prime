# AIRL vs Prime Comparison

## AIRL Concept → Prime Equivalent / Missing → Implementation → Benefit

### 1. Typed program graph (nodes + edges)
AIRL concept: 16 node types, explicit types on every node, DAG
Prime equivalent: Entities + Relations, symbol kinds, RelationKind
Missing: Node-level computation graph, dataflow edges inside functions
Potential: Keep symbol graph, add optional semantic nodes for functions (dataflow)
Expected benefit: Better reasoning about dataflow and effects inside functions without full IR

### 2. Effects system
AIRL concept: Explicit effects Pure/IO/Read/Write/Fail/Diverge, lattice, checked
Prime equivalent: None
Missing: Effect metadata on functions
Potential: Add `effects: Vec<Effect>` to Entity for callable kinds, derive from static analysis
Benefit: Deterministic validation of forbidden effects, capabilitiy checking

### 3. Semantic patches
AIRL concept: Invertible semantic patch ops (ReplaceNode, AddFunction, RenameSymbol) with preconditions, evidence, constraints_checked
Prime equivalent: Source editing via CLI, no semantic layer
Missing: Semantic editing layer
Potential: Implement experimental `SemanticPatch` type operating on KnowledgeGraph, project back to source via AST
Benefit: Agent can edit semantics safely, with validation

### 4. Provenance & evidence
AIRL concept: Evidence per patch (test result), provenance per node
Prime equivalent: Provenance enum + confidence, AgentProvenance in envelope
Missing: Structured evidence chain linking facts to source/analysis/runtime
Potential: Extend Relation provenance with evidence records
Benefit: Source-free confidence, auditability

### 5. Content-addressed versioning
AIRL concept: SHA-256 VersionId per module, patch DAG
Prime equivalent: ContentHash per file, Project content_hash
Missing: Graph-level versioning, patch history
Potential: Add Version struct for KnowledgeGraph snapshots
Benefit: Reproducibility, incremental updates

### 6. Constraints & validation
AIRL concept: Type checker + effect checker + constraints (max complexity)
Prime equivalent: None
Missing: Validation layer
Potential: Add `Constraint` engine checking architecture boundaries, missing tests, type mismatches
Benefit: Deterministic enforcement

### 7. Projections (IR -> TS/Python)
AIRL concept: Project IR to human code
Prime equivalent: Export to JSON/msgpack/DOT
Missing: Projection back to source from semantic model
Potential: Keep source as source of truth, projection is read-only for now
Benefit: No breakage

### 8. Binary/canonical serialization
AIRL concept: Canonical JSON, deterministic IDs, binary storage
Prime equivalent: Compact binary with compression, mmap
Missing: Canonical serialization for semantic IR
Potential: Already have; add semantic IR binary format
Benefit: Fast load, hashing

### 9. Query API
AIRL concept: HTTP API with 15 endpoints, search/call-graph/effects
Prime equivalent: MCP 7 tools, QueryEngine
Missing: Dataflow/effects queries
Potential: Extend ToolExecutor with semantic queries
Benefit: Agent navigation

### 10. Interpreter / compiler
AIRL concept: Interpreter + Cranelift JIT + WASM
Prime equivalent: None
Missing: Not needed for Prime
Decision: Reject for Prime; Prime is repository analysis, not execution

## Adoption decisions

Ideas Prime should adopt:
- Effects metadata on entities
- Semantic patch engine (experimental)
- Structured evidence/provenance
- Constraint validation layer
- Content-addressed versioning for graph snapshots

Ideas Prime should modify:
- Typed graph → keep symbol graph, add optional semantic layer
- Patch invertibility → start with validation, invertibility later
- Projections → keep source authoritative, projection read-only

Ideas unnecessary for Prime:
- Full interpreter/compiler
- Cranelift JIT/WASM
- Language-specific IR nodes for computations

Ideas experimental:
- Dataflow analysis inside functions
- Semantic compression
- CRDT distributed knowledge

## Key differences
AIRL is authoring target for new programs. Prime is analysis of existing repositories. Prime must preserve source fidelity and support many languages without rewriting them.
