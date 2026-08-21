---
title: Representation
---

# Representation

Prime's physical representation — the binary format of the knowledge artifact.

## Design Goals

| Goal | Target |
|------|--------|
| Artifact size | < 2x source |
| Load time (mmap) | < 10ms |
| Point query latency | < 500µs p95 |
| Memory footprint | < 2x artifact size |
| Concurrency | Read-only, multi-process |
| Portability | Linux, macOS, Windows |

## Binary Format

```
HEADER (magic, version, schema hash, index offsets)
INDEX SECTION (name → offset mappings)
DATA SECTION (entities, relations, strings, bitmaps)
FOOTER (checksum, artifact hash)
```

### Header (64 bytes)
```
Magic: "PRIM" (4 bytes)
Version: u16 (2 bytes)
Schema hash: blake3 (32 bytes)
Index offset: u64 (8 bytes)
Data offset: u64 (8 bytes)
Flags: u16 (2 bytes)
Reserved: 10 bytes
```

### String Interning
- Global string table with offsets
- Qualified names, file paths, documentation
- Variable-length encoding (LEB128)
- Deduplicated across artifact

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

### Index Structures

| Index | Format |
|-------|--------|
| Name index | Perfect hash → entity offset |
| Prefix index | FST (Finite State Transducer) |
| Keyword index | Inverted index (token → varint[]) |
| Relation index | Adjacency lists (outgoing/incoming) |
| Dependency bitmaps | Roaring bitmaps per entity |

## Memory Mapping Strategy

- **Read-only mmap** — OS manages paging
- **Huge pages** — Reduce TLB misses (2MB/1GB)
- **Prefetch hints** — Sequential index scans
- **Zero-copy deserialization** — Direct struct access

## Compression

| Component | Technique |
|-----------|-----------|
| Strings | Dictionary + LEB128 |
| Integers | LEB128 varint |
| Bitmaps | Roaring (run-length + array) |
| Offsets | Delta + LEB128 |
| Relations | Sorted by from, delta-encoded |

## Schema Evolution

- **Versioned schema** — Header includes schema hash
- **Forward compatibility** — Unknown fields ignored
- **Backward compatibility** — Default values for missing fields
- **Migration tool** — `prime migrate` for major versions

## Open Questions

- Columnar vs row-oriented for mixed query workloads?
- Compression: zstd vs lz4 vs none (mmap favors uncompressed)?
- CAS: worth it for single-node, or only for distributed?
- Schema evolution: how to handle format changes?

## Next

- [Artifact](./artifact.md)
- [Language Model](./language-model.md)
- [Provenance](./provenance.md)