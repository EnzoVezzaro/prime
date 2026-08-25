# Agent-Native Semantic Representation Design

## Principles
- Additive layer over existing KnowledgeGraph, not replacement
- Evidence-backed facts only
- Multi-view: source, structural, semantic graph, agent serialization, binary canonical
- Token-efficient serialization for LLMs
- Deterministic validation separate from probabilistic reasoning

## Representation Layers

### 1. Source representation (existing)
Tree-sitter AST → Entities + Relations. Source of truth for projection.

### 2. Structural representation
Symbol tree with containment, imports/exports. Already present.

### 3. Semantic graph representation (new)
Agent-Native IR built from KnowledgeGraph + enriched metadata.

### 4. Compact agent-facing serialization
Optimized for token efficiency, not human readability.

### 5. Binary/canonical
Deterministic binary for storage/caching/hashing.

## Semantic Model

### Program structure nodes
Repository, Package, Directory, Module, Class, Interface, Function, Method, Variable, Type, Constant
Mapped to SymbolKind.

### Relationships
contains, imports, exports, calls, called_by, depends_on, inherits, implements, reads, writes, produces, consumes, controls, data_flows_to
Extend RelationKind with data_flows_to, controls.

### Semantic metadata on Entity
inputs: Vec<TypeRef>
outputs: Vec<TypeRef>
effects: Vec<Effect>
capabilities: Vec<String>
permissions: Vec<String>
invariants: Vec<String>
preconditions: Vec<String>
postconditions: Vec<String>
errors: Vec<String>
external_resources: Vec<String>
external_services: Vec<String>

### Repository-level knowledge
architecture_boundaries: Vec<Boundary>
service_boundaries
APIs
databases
queues
external_providers
configuration
tests
documentation_links

### Provenance & Evidence
Every fact links to:
source_location: file_id + range
analysis_type: AST/Static/Runtime/Test/Benchmark/Git
confidence: Exact/High/Medium/Low
evidence_refs: [EvidenceId]

Evidence types:
StaticAnalysis, ASTAnalysis, RuntimeObservation, TestResult, BenchmarkResult, CommitInfo

## Data structures (proposed extensions to prime-core)

```rust
struct SemanticEntity {
    base: Entity,
    inputs: Vec<TypeRef>,
    outputs: Vec<TypeRef>,
    effects: Vec<Effect>,
    capabilities: Vec<Capability>,
    contracts: Contracts,
    evidence: Vec<EvidenceRef>,
}

enum Effect { Pure, IO, Read(Resource), Write(Resource), Allocate, Fail(ErrorType), Diverge }

struct Contracts { preconditions: Vec<String>, postconditions: Vec<String>, invariants: Vec<String> }

struct Evidence { id: EvidenceId, kind: EvidenceKind, source: String, confidence: Confidence, timestamp: u64 }
```

## Serialization views

### Agent view
Compact line-oriented format, e.g.:
```
E func:42 AuthService.login | in:UserCredentials | out:AuthToken | eff:IO,Read(db) | calls:AuthService.validate | reads:db.users
R calls:42->43
```
Goal: high semantic density per token.

### JSON view
Standard JSON for tooling.

### Binary view
Canonical deterministic serialization for hashing/dedup.

## Query layer extensions
get_repository, get_module, get_function, get_symbol
get_dependencies, get_callers, get_callees
get_dataflow, get_effects, get_capabilities, get_constraints
get_impact, get_tests, get_evidence, get_architecture

## Semantic patches (experimental)
Operations:
AddFunction, RemoveFunction, RenameSymbol, ChangeType, AddDependency, RemoveDependency, ModifyCall, AddContract, RemoveContract, ChangeEffect, MoveComponent, ChangeConfiguration, ModifyDataflow

Patch struct:
target, operation, precondition, resulting_state, evidence, validation_results, affected_nodes, affected_tests

## Validation layer
Checks:
invalid_dependency, type_mismatch, forbidden_effect, missing_capability, broken_invariant, invalid_architecture_boundary, missing_required_test, unexpected_external_side_effect

## Tokenization study
Compare JSON, compact JSON, line-oriented, S-expression, custom notation. Measure tokens per fact, per dependency, per relationship.

## Compatibility
Repository → existing parsers → KnowledgeGraph → Agent-Native IR → query/agent → semantic patch → source projection (read-only initially)

No removal of source path.
