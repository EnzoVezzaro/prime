---
title: Indexing
---

# Indexing Research

## Index Types for Agent Queries

| Index | Query Type | Latency Target |
|-------|------------|----------------|
| Exact name | `prime_lookup` | < 50µs |
| Prefix | `prime_search` (prefix) | < 100µs |
| Keyword | `prime_search` (keyword) | < 200µs |
| Outgoing edges | `prime_context` (calls) | < 500µs |
| Incoming edges | `prime_context` (callers) | < 500µs |
| Transitive closure | `prime_dependencies`, `prime_impact` | < 1ms |
| Architecture | `prime_architecture` | < 2ms |

## Index Structures Under Investigation

### 1. Inverted Index (Keywords)
- Token → entity ID postings list
- TF-IDF or BM25 scoring
- Compressed with Elias-Fano or varint

### 2. Prefix Trie / FST
- Character-level trie or Finite State Transducer
- Fast prefix matching
- Minimal perfect hash for exact lookup

### 3. Name Hash Map
- Qualified name → entity ID
- Perfect hash or Robin Hood hashing
- O(1) exact lookup

### 4. Graph Indexes
- **Adjacency lists** — outgoing/incoming per entity
- **Roaring bitmaps** — transitive closure per entity
- **k²-trees** — compressed adjacency for massive graphs

### 5. Learned Indexes
- Replace B-trees with ML models
- CDF-based position prediction
- Potential for 10x smaller, 2x faster

## Index Construction

| Phase | Input | Output |
|-------|-------|--------|
| Parse | Source files | AST, symbols |
| Extract | AST | Entities, relations |
| Resolve | Symbols | Qualified names, types |
| Index | Entities, relations | All index structures |
| Serialize | Indexes | Binary artifact |

## Incremental Updates

- **File-level**: Re-parse changed files, update affected entities
- **Symbol-level**: Track symbol identity across versions (SCIP-style)
- **Index-level**: Delta-encode index changes
- **Invalidation**: Remove stale entries, add new ones

## Open Questions

- Learned indexes: worth the complexity for our access patterns?
- Roaring bitmaps vs Elias-Fano for transitive closure?
- How to handle cross-language symbol identity in indexes?
- Optimal index granularity (per-file, per-module, global)?

## Next

- [Storage](./storage.md)
- [Distributed Systems](./distributed-systems.md)
- [Specification: Representation](../specification/representation.md)