# Storage & Representation Synthesis

**Confidence:** OBSERVATION (documented approaches), HYPOTHESIS (Prime recommendations)
**Primary Sources:** SCIP, LSIF, Kythe, Code-Graph-RAG, succinct data structures research
**Last Updated:** August 2026

## Executive Summary

Code intelligence systems use diverse storage strategies: JSON (LSIF), Protobuf (SCIP), graph databases (Code-Graph-RAG/Memgraph), and custom binary (Prime). This synthesis recommends a **layered storage architecture** for Prime: compact binary for fast access, optional graph database for complex queries.

## Storage Approaches Compared

### 1. JSON/NDJSON (LSIF)

**Format:** Newline-delimited JSON

```json
{"id":1,"type":"vertex","label":"document","uri":"file:///src/main.rs","languageId":"rust"}
{"id":2,"type":"vertex","label":"range","start":{"line":10,"character":0},"end":{"line":10,"character":5}}
{"id":3,"type":"edge","label":"textDocument/definition","outV":2,"inV":4}
```

| Pros | Cons |
|------|------|
| Human-readable | Verbose (5-10x source size) |
| Easy to debug | Slow to parse |
| Streaming-friendly | No random access |
| Language-agnostic | No compression |

**Storage Size:** 5-10x source code
**Query Latency:** 100ms-1s (requires full parse)

### 2. Protobuf (SCIP)

**Format:** Protocol Buffers with TLV (Type-Length-Value)

```protobuf
message Document {
  string relative_path = 1;
  repeated Occurrence occurrences = 2;
  repeated SymbolInformation symbol_information = 3;
}
```

| Pros | Cons |
|------|------|
| Compact binary | Not human-readable |
| Schema evolution | Requires codegen |
| Streaming reads | No random access without index |
| Good compression (10-20%) | Complex parsing |

**Storage Size:** 1-2x source code (after compression)
**Query Latency:** 10-100ms (requires index)

### 3. Graph Database (Code-Graph-RAG/Memgraph)

**Format:** Property graph with Cypher queries

```cypher
CREATE (m:Module {qualified_name: "auth.service"})
CREATE (f:Function {qualified_name: "auth.service.login"})
CREATE (m)-[:DEFINES]->(f)
```

| Pros | Cons |
|------|------|
| Rich queries (Cypher) | High memory overhead |
| Flexible schema | Slow startup |
| Mature tooling | Not embeddable |
| ACID transactions | Overkill for read-only |

**Storage Size:** 10-50x source code
**Query Latency:** 10-100ms (indexed)

### 4. Custom Binary + mmap (Prime)

**Format:** Custom compact binary with memory mapping

```
[Header: 64 bytes]
[Entity Index: N × 8 bytes]
[Entity Data: variable]
[Relation Index: M × 12 bytes]
[Relation Data: variable]
[String Table: compressed]
```

| Pros | Cons |
|------|------|
| Very fast (mmap) | Not human-readable |
| Compact (succinct structures) | Requires custom parser |
| Random access | Schema evolution harder |
| Zero-copy reads | Debugging harder |

**Storage Size:** 0.3-1x source code (target)
**Query Latency:** 1-10ms (mmap)

### 5. SQLite (SCIP experimental, OpenGrok)

**Format:** Relational tables with indexes

```sql
CREATE TABLE symbols (id INTEGER, name TEXT, kind TEXT, path TEXT);
CREATE TABLE references (source_id INTEGER, target_id INTEGER, kind TEXT);
CREATE INDEX idx_symbols_name ON symbols(name);
```

| Pros | Cons |
|------|------|
| SQL queries | Moderate overhead |
| ACID transactions | Not as fast as mmap |
| Mature tooling | Schema migrations |
| Good compression | Not embedded by default |

**Storage Size:** 1-3x source code
**Query Latency:** 10-50ms (indexed)

## Recommended Architecture: Layered Storage

### Layer 1: Compact Binary (Primary)

**Purpose:** Fast reads, low memory, agent queries

```
┌─────────────────────────────────────────────────────────────┐
│                    Prime Binary Format                        │
├─────────────────────────────────────────────────────────────┤
│  Header (64 bytes)                                          │
│  - Magic number, version, flags                             │
│  - Entity count, relation count                             │
│  - Offset to each section                                   │
├─────────────────────────────────────────────────────────────┤
│  String Table (compressed)                                  │
│  - All strings (names, paths, docs)                         │
│  - Dictionary-encoded, zstd compressed                      │
├─────────────────────────────────────────────────────────────┤
│  Entity Section                                             │
│  - Sorted by qualified name (binary search)                 │
│  - Fixed-size records + variable data                       │
│  - Entity ID = index into this section                      │
├─────────────────────────────────────────────────────────────┤
│  Relation Section                                           │
│  - Sorted by source entity (delta encoding)                 │
│  - Variable-length records (varint encoded)                 │
│  - Bitmap index for relation kinds                          │
├─────────────────────────────────────────────────────────────┤
│  Index Section                                              │
│  - Name → Entity ID mapping (hash table)                    │
│  - File → Entity IDs mapping (inverted index)               │
│  - Relation kind → relations mapping (bitmap index)         │
└─────────────────────────────────────────────────────────────┘
```

**Target Metrics:**
- Storage size: 0.3-1x source code
- Query latency: 1-10ms (mmap)
- Memory usage: 10-50MB per 10K LOC

### Layer 2: Optional SQLite (Extended Queries)

**Purpose:** Complex queries, analytics, export

```
┌─────────────────────────────────────────────────────────────┐
│                    SQLite Extension                          │
├─────────────────────────────────────────────────────────────┤
│  Tables:                                                    │
│  - entities (id, name, kind, file, line, column)            │
│  - relations (source, target, kind, properties)             │
│  - files (path, language, hash, size)                       │
│  - symbols (entity_id, documentation, metadata)             │
├─────────────────────────────────────────────────────────────┤
│  Indexes:                                                   │
│  - idx_entities_name (name)                                 │
│  - idx_entities_file (file)                                 │
│  - idx_relations_source (source)                            │
│  - idx_relations_target (target)                            │
│  - idx_relations_kind (kind)                                │
└─────────────────────────────────────────────────────────────┘
```

**When to Use:**
- Complex graph queries (multiple joins)
- Analytics (statistics, reports)
- Export to other formats
- Debugging and inspection

### Layer 3: Optional Graph DB (Advanced)

**Purpose:** Cypher/Gremlin queries, distributed access

**When to Use:**
- Multi-user concurrent access
- Complex graph algorithms (centrality, clustering)
- Integration with existing graph tools
- Distributed knowledge sharing

## Encoding Strategies

### String Encoding

**Current (Hypothetical):**
```
[entity_name: null-terminated string]
```
Size: len(name) + 1 bytes

**Optimized:**
```
[entity_name_idx: varint]  // index into string table
```
Size: 1-4 bytes

**String Table:**
```
[entry_count: u32]
[entries: sorted strings]
[hash_table: for lookup]
```

### Entity Encoding

**Fixed-Size Record:**
```
[entity_id: u32]           // 4 bytes
[name_idx: varint]         // 1-4 bytes
[kind: u4]                 // 0.5 bytes (16 kinds)
[file_idx: varint]         // 1-4 bytes
[start_line: varint]       // 1-4 bytes
[end_line: varint]         // 1-4 bytes
[flags: u8]                // 1 byte
```
Total: ~14-21 bytes per entity

**Comparison:**
- JSON: ~200-500 bytes per entity
- Protobuf: ~50-100 bytes per entity
- Prime binary: ~14-21 bytes per entity (target)

### Relation Encoding

**Delta Encoding:**
```
[source_delta: varint]     // delta from previous source (often 0)
[target_delta: varint]     // delta from source (often small)
[kind: u4]                // 0.5 bytes (16 kinds)
[confidence: u2]           // 0.25 bytes (4 levels)
[flags: u2]                // 0.25 bytes
```
Total: ~3-10 bytes per relation

**Bitmap Index for Relation Kinds:**
```
[bitmap: ceil(entity_count * relation_kind_count / 8) bytes]
```
Example: 10K entities × 20 kinds = 25KB bitmap

### Compression Strategy

| Component | Compression | Ratio | Notes |
|-----------|-------------|-------|-------|
| String table | zstd | 3-5x | High compression |
| Entity section | None | 1x | Already compact |
| Relation section | lz4 | 1.5-2x | Fast decompression |
| Bitmap index | None | 1x | Already compact |
| **Overall** | **zstd** | **2-4x** | **Target: 0.3-1x source** |

## Performance Projections

### Storage Size

| Codebase | Source Size | JSON | Protobuf | Prime Binary |
|----------|-------------|------|----------|--------------|
| Small (10K LOC) | 300KB | 3MB | 600KB | 100-300KB |
| Medium (100K LOC) | 3MB | 30MB | 6MB | 1-3MB |
| Large (1M LOC) | 30MB | 300MB | 60MB | 10-30MB |

### Query Latency

| Query Type | JSON | Protobuf | SQLite | Prime Binary |
|-----------|------|----------|--------|--------------|
| Symbol lookup | 100ms | 10ms | 5ms | 1ms |
| Find references | 500ms | 50ms | 20ms | 5ms |
| Call graph traversal | 1s | 100ms | 50ms | 10ms |
| Impact analysis | 2s | 200ms | 100ms | 20ms |

### Memory Usage

| Codebase | JSON | Protobuf | SQLite | Prime Binary (mmap) |
|----------|------|----------|--------|---------------------|
| Small (10K LOC) | 10MB | 5MB | 5MB | 1MB |
| Medium (100K LOC) | 100MB | 50MB | 50MB | 10MB |
| Large (1M LOC) | 1GB | 500MB | 500MB | 100MB |

## Implementation Roadmap

### Phase 1: Basic Binary Format (2-4 weeks)

- [ ] Design header format
- [ ] Implement string table with zstd compression
- [ ] Implement entity section with fixed-size records
- [ ] Implement relation section with delta encoding
- [ ] Implement basic indexes (name → entity, file → entities)

### Phase 2: Advanced Encoding (2-4 weeks)

- [ ] Implement varint encoding
- [ ] Implement bitmap index for relation kinds
- [ ] Optimize memory layout for cache efficiency
- [ ] Implement mmap access layer

### Phase 3: Compression Optimization (1-2 weeks)

- [ ] Benchmark zstd vs lz4 vs none
- [ ] Tune compression levels
- [ ] Implement streaming compression for large files

### Phase 4: SQLite Extension (2-4 weeks)

- [ ] Design SQLite schema
- [ ] Implement import from binary format
- [ ] Implement export to binary format
- [ ] Add complex query support

## Open Questions

1. **OPEN QUESTION:** What's the optimal record size for cache efficiency? 32 bytes (L1 cache line) vs 64 bytes (L2 cache line) vs variable?

2. **OPEN QUESTION:** How to handle schema evolution in binary format? Version field + forward/backward compatibility rules?

3. **OPEN QUESTION:** Should Prime support multiple binary format versions simultaneously (for migration)?

4. **OPEN QUESTION:** How to benchmark storage format performance fairly? Need to control for implementation quality, not just format design.

## Confidence Assessment

| Claim | Confidence |
|-------|------------|
| Binary format is faster than JSON/Protobuf | **FACT** (well-established) |
| mmap enables zero-copy reads | **FACT** (OS documentation) |
| 0.3-1x source size is achievable | **HYPOTHESIS** (requires implementation) |
| 1-10ms query latency is achievable | **HYPOTHESIS** (requires benchmarking) |
| Layered architecture provides flexibility | **INFERENCE** (based on use cases) |
| String table compression saves 3-5x | **OBSERVATION** (typical text compression) |
