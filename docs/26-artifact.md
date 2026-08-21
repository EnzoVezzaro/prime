---
title: Artifact
---

# Artifact

The Prime artifact is the single logical knowledge representation produced from a codebase.

## Artifact Properties

| Property | Description |
|----------|-------------|
| **Single file** | One `.prime` file per repository |
| **Content-addressed** | Identified by blake3 hash of content |
| **Immutable** | Never modified after creation |
| **Versioned** | Schema version in header |
| **Self-describing** | Contains schema hash, indexes |
| **Portable** | Runs on Linux, macOS, Windows |

## Artifact Lifecycle

```
Source code
    │
    ▼
prime build
    │
    ▼
.prime artifact (content-addressed)
    │
    ├──► prime serve (MCP server)
    ├──► prime query (CLI)
    ├──► prime export (JSON, DOT, etc.)
    └──► Distribution (registry, P2P, CI)
```

## Artifact Structure

```
my-project.prime
├── header (64 bytes)
├── index_section/
│   ├── name_index (perfect hash)
│   ├── prefix_index (FST)
│   ├── keyword_index (inverted)
│   ├── relation_index (adjacency)
│   └── dependency_bitmaps (roaring)
├── data_section/
│   ├── string_table (interned)
│   ├── entities (compact structs)
│   ├── relations (compact structs)
│   ├── files (compact structs)
│   └── modules (compact structs)
└── footer (checksum, artifact hash)
```

## Content Addressing

- **Hash**: BLAKE3 (fast, parallelizable, 256-bit)
- **Identifier**: `prime:<blake3_hash>`
- **Verification**: Hash checked on load
- **Deduplication**: Identical artifacts = same hash

## Versioning

| Component | Versioning |
|-----------|------------|
| Schema | Semantic (major.minor) |
| Artifact | Content hash (immutable) |
| Schema hash | In header, verified on load |
| Migration | `prime migrate` for major versions |

## Distribution

| Channel | Mechanism |
|---------|-----------|
| Local | File system |
| CI/CD | Upload to registry |
| Registry | HTTP API (content-addressed) |
| P2P | libp2p, content-addressed |
| Registry API | `GET /artifacts/{hash}` |

## Integrity

- **Footer checksum**: CRC32C of entire artifact
- **Artifact hash**: BLAKE3 of content (excludes footer)
- **Schema hash**: In header, verified against known schemas
- **Verification**: `prime verify <artifact>`

## Open Questions

- Streaming load for huge artifacts (>10GB)?
- Partial artifact fetch (index only)?
- Delta artifacts for incremental updates?
- Artifact signing and verification?

## Next

- [Language Model](./language-model.md)
- [Provenance](./provenance.md)
- [Confidence](./confidence.md)