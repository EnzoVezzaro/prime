---
title: Storage
---

# Storage Research

## Storage Requirements

| Requirement | Target |
|-------------|--------|
| Artifact size | < 2x source |
| Load time (mmap) | < 10ms |
| Point query latency | < 500µs p95 |
| Memory footprint | < 2x artifact size |
| Concurrency | Read-only, multi-process |
| Portability | Linux, macOS, Windows |

## Storage Backends Under Evaluation

### 1. Custom Binary + mmap (Current)
- Flat binary layout with offsets
- mmap for zero-copy access
- OS page cache handles hot data
- Pros: Simple, fast, OS-managed caching
- Cons: No compression, fixed schema

### 2. Columnar (Parquet/Arrow)
- Column-oriented for analytical queries
- Dictionary encoding, RLE
- Pros: Compression, SIMD-friendly
- Cons: Row reconstruction overhead for point queries

### 3. Embedded Key-Value (SQLite, LMDB, RocksDB)
- ACID, transactions
- Pros: Mature, flexible queries
- Cons: Overhead for read-only workloads

### 4. Content-Addressed (CAS + Merkle DAG)
- Hash-addressed blocks
- Deduplication, sync-friendly
- Pros: Distributed, verifiable
- Cons: Indirection overhead

### 5. Succinct Structures (Custom)
- Elias-Fano, rank/select, Roaring
- Pros: Minimal space, fast queries
- Cons: Complex, schema-coupled

## Binary Format Design

```
HEADER (magic, version, schema hash, index offsets)
INDEX SECTION (name → offset mappings)
DATA SECTION (entities, relations, strings, bitmaps)
FOOTER (checksum, artifact hash)
```

### String Interning
- Global string table with offsets
- Qualified names, file paths, documentation
- Variable-length encoding (LEB128)

### Entity Encoding
```
Entity {
  id: varint
  kind: u8
  name_offset: varint
  qualified_name_offset: varint
  file_id: varint
  range_start: varint
  range_end: varint
  language: u8
  confidence: u8
  signature_offset: varint (optional)
  doc_offset: varint (optional)
  children_count: varint
  children_ids: varint[]
  relations_count: varint
  relations: Relation[]
}
```

### Relation Encoding
```
Relation {
  from: varint
  to: varint
  kind: u8
  confidence: u8
  provenance: u8
}
```

## Memory Mapping Strategy

- **Read-only mmap** — OS manages paging
- **Huge pages** — Reduce TLB misses
- **Prefetch hints** — Sequential index scans
- **No copy deserialization** — Zero-copy access

## Open Questions

- Columnar vs row-oriented for mixed query workloads?
- Compression: zstd vs lz4 vs none (mmap favors uncompressed)?
- CAS: worth it for single-node, or only for distributed?
- Schema evolution: how to handle format changes?

## Next

- [Distributed Systems](./distributed-systems.md)
- [Cryptography](./cryptography.md)
- [Specification: Artifact](../specification/artifact.md)