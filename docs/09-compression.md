---
title: Compression
---

# Compression Research

## Semantic Compression vs Syntactic Compression

Traditional compression makes the same information smaller. Prime's compression removes what agents don't need while preserving what they do.

```
compression:
    make the same information smaller

Prime:
    remove information that is unnecessary
    while preserving what the agent needs
```

## Compression Techniques Under Investigation

### 1. Grammar-Based Compression
- Find repeated patterns in the knowledge graph
- Replace with references to a grammar
- Effective for: repeated relation patterns, common type signatures

### 2. Delta Encoding
- Store only changes between versions
- Effective for: incremental updates, version history

### 3. Variable-Length Integers
- LEB128, varint for entity IDs, offsets
- Effective for: large graphs with sparse IDs

### 4. Dictionary Encoding
- Intern strings (qualified names, file paths)
- Effective for: repeated identifiers

### 5. Succinct Data Structures
- Rank/select bitvectors for adjacency
- Elias-Fano for monotonic sequences
- Roaring bitmaps for transitive closure
- Effective for: large-scale graph queries

### 6. Graph Compression
- WebGraph-style techniques
- k²-trees for adjacency matrices
- Effective for: massive graphs (1M+ nodes)

## What NOT to Compress

- **Provenance metadata** — needed for trust
- **Confidence levels** — needed for agent reasoning
- **Source locations** — needed for escalation
- **Contract signatures** — high utility per byte

## Compression Targets

| Metric | Target |
|--------|--------|
| Artifact/Source ratio | < 2x (currently ~1.2x) |
| Deserialization time | < 10ms for 100K entities |
| Query latency (p95) | < 500µs |
| Memory overhead | < 2x artifact size |

## Tradeoffs

| Technique | Compression | Query Speed | Implementation Complexity |
|-----------|-------------|-------------|---------------------------|
| Dictionary | 2-3x | Fast | Low |
| Grammar | 3-10x | Medium | High |
| Succinct | 2-5x | Fast (rank/select) | Medium |
| Delta | 10-100x (incremental) | N/A (updates only) | Medium |

## Next

- [Information Theory](./information-theory.md)
- [Indexing](./indexing.md)
- [Specification: Representation](../specification/representation.md)