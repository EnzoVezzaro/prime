# Relationship Model Synthesis

**Confidence:** OBSERVATION (documented patterns), HYPOTHESIS (Prime recommendations)
**Primary Sources:** Code-Graph-RAG, SCIP, LSIF, Kythe, Joern
**Last Updated:** August 2026

## Executive Summary

Code intelligence systems converge on ~8 core relationship types, with specialized systems adding 10+ more. Prime currently implements 7 relationship types. This synthesis defines the complete relationship model Prime should adopt.

## Core Relationship Types (Universal)

These appear in 4+ systems and are essential for any code intelligence:

| # | Relationship | Definition | Example | Source Systems |
|---|-------------|------------|---------|----------------|
| 1 | `CONTAINS` | Structural containment | File → Function | All |
| 2 | `DEFINES` | Symbol definition | Module → Function | All |
| 3 | `CALLS` | Function call | Function → Function | SCIP, Kythe, Code-Graph-RAG |
| 4 | `DEPENDS_ON` | Module dependency | Module → Module | Prime, Code-Graph-RAG |
| 5 | `INHERITS` | Class inheritance | Class → Class | SCIP, Kythe, Code-Graph-RAG |
| 6 | `IMPLEMENTS` | Interface implementation | Class → Interface | SCIP, Kythe, Code-Graph-RAG |
| 7 | `IMPORTS` | Module import | Module → Module | Kythe, Code-Graph-RAG |

**Prime Status:** ✅ All 7 implemented

## Extended Relationship Types (High Value)

These appear in 2-3 systems and significantly enhance query capabilities:

| # | Relationship | Definition | Example | Source Systems | Priority |
|---|-------------|------------|---------|----------------|----------|
| 8 | `OVERRIDES` | Method override | Method → Method | Kythe, Code-Graph-RAG | P1 |
| 9 | `EXPORTS` | Public API surface | Module → Function | Code-Graph-RAG | P1 |
| 10 | `REFERENCES` | Non-call reference | Function → Function | Code-Graph-RAG | P1 |
| 11 | `INSTANTIATES` | Class construction | Function → Class | Code-Graph-RAG | P2 |
| 12 | `READS_FROM` | I/O read | Function → Resource | Code-Graph-RAG | P2 |
| 13 | `WRITES_TO` | I/O write | Function → Resource | Code-Graph-RAG | P2 |

**Prime Status:** ❌ None implemented

## Advanced Relationship Types (Specialized)

These appear in 1-2 systems and enable advanced analysis:

| # | Relationship | Definition | Example | Source Systems | Priority |
|---|-------------|------------|---------|----------------|----------|
| 14 | `FLOWS_TO` | Data flow | Function → Function | Code-Graph-RAG, Kythe | P2 |
| 15 | `IMPLEMENTS_MODULE` | Module-level impl | ModuleInterface → ModuleImpl | Code-Graph-RAG | P3 |
| 16 | `DEFINES_METHOD` | Class method ownership | Class → Method | Code-Graph-RAG | P3 |
| 17 | `IMPLEMENTS_PATTERN` | Pattern match | Module → Pattern | Code-Graph-RAG | P3 |
| 18 | `HAS_SMELL` | Code smell | Module → CodeSmell | Code-Graph-RAG | P3 |
| 19 | `HAS_VULNERABILITY` | Security issue | Module → SecurityIssue | Code-Graph-RAG | P3 |

**Prime Status:** ❌ None implemented

## Relationship Properties

### Current Prime Properties

```rust
pub struct Relation {
    pub source: EntityId,
    pub target: EntityId,
    pub kind: RelationKind,
    pub confidence: Confidence,
    pub provenance: Provenance,
}
```

### Recommended Additional Properties

Based on Code-Graph-RAG's `FLOWS_TO` model:

```rust
// For FLOWS_TO relationships
pub struct FlowProperties {
    pub kind: FlowKind,      // resource, arg, return
    pub via: FlowVia,        // arg:<index>, kw:<name>, return
    pub coverage: FlowCoverage, // found, no_flow, unknown
}

// For OVERRIDES relationships  
pub struct OverrideProperties {
    pub is_virtual: bool,
    pub vtable_slot: Option<u32>,
}

// For EXPORTS relationships
pub struct ExportProperties {
    pub visibility: Visibility,  // public, private, protected
    pub re_export: bool,        // re-exported from another module
}
```

## Relationship Query Patterns

### Pattern 1: Impact Analysis

```
Given: Function F
Query: What is affected if F changes?
Traversal: F → CALLS (callers) → DEFINES (containing modules) → DEPENDS_ON (dependent modules)
```

### Pattern 2: Dead Code Detection

```
Given: Module M
Query: Is M used?
Traversal: M → EXPORTS (exported entities) → REFERENCES (all references)
If no references to exported entities → dead code
```

### Pattern 3: Data Flow Analysis

```
Given: Source S (e.g., user input)
Query: Where does data from S flow?
Traversal: S → FLOWS_TO (with kind=via) → transitive closure
```

### Pattern 4: Override Chain

```
Given: Method M
Query: What overrides M?
Traversal: M → OVERRIDES (reverse) → all overriding methods
```

## Relationship Encoding in Prime Binary Format

### Current Encoding (Hypothetical)

```
[entity_id: u64][relation_kind: u8][target_id: u64][confidence: u8]
```
Size: 18 bytes per relation

### Optimized Encoding (Recommended)

Using varint + dictionary encoding:

```
[entity_id: varint][relation_kind_idx: u4][target_id: delta_varint][confidence: u2]
```
Size: ~6-10 bytes per relation (estimated 50% savings)

### Compression Strategy

1. **Sort by source entity** — enables delta encoding on source IDs
2. **Dictionary encode relation kinds** — 20 types → 5 bits
3. **Delta encode target IDs** — targets are often close to source
4. **Bitmap for confidence** — 4 levels → 2 bits

## Implementation Roadmap

| Phase | Relationships | Effort | Impact |
|-------|--------------|--------|--------|
| **Phase 1** | OVERRIDES, EXPORTS | Low | High (dead code, override chains) |
| **Phase 2** | REFERENCES, INSTANTIATES | Low | Medium (callback tracking) |
| **Phase 3** | FLOWS_TO (basic) | Medium | High (data flow) |
| **Phase 4** | FLOWS_TO (full) + Resource nodes | High | High (taint analysis) |
| **Phase 5** | Pattern/CodeSmell/SecurityIssue | Low | Medium (findings integration) |

## Confidence Assessment

| Claim | Confidence |
|-------|------------|
| 7 core relationships are universal | **FACT** (documented in 4+ systems) |
| OVERRIDES + EXPORTS are high-value | **OBSERVATION** (Code-Graph-RAG demonstrates) |
| FLOWS_TO enables data flow analysis | **FACT** (Code-Graph-RAG, Kythe) |
| Binary encoding can achieve 50% savings | **HYPOTHESIS** (requires benchmarking) |
| Phase 1 is low-effort, high-impact | **INFERENCE** (simple additions to schema) |
