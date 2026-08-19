# storage

## Purpose

Research on storage systems and formats for codebase knowledge artifacts.

## Responsibilities

- Research databases (SQLite, DuckDB, RocksDB, LMDB, custom binary, columnar)
- Research binary formats (protobuf, FlatBuffers, Cap'n Proto, MessagePack, CBOR, Apache Arrow)
- Research mmap, page cache, page faults, SSD/NVMe behavior
- Research columnar storage and custom storage designs

## Ownership

Owner: research team

## Inputs

- Database benchmarks and documentation
- Binary format specifications (protobuf, FlatBuffers, Cap'n Proto, Apache Arrow)
- OS/mmapping documentation (mmap, page cache, NVMe)
- Columnar format specifications (Parquet, Feather, ORC)

## Outputs

- SPECS/storage/databases.md
- SPECS/storage/binary-formats.md
- SPECS/storage/mmap.md
- SPECS/storage/columnar.md
- SPECS/storage/custom-storage.md

## Dependencies

- SPECS/compression/ (compression integration)
- SPECS/indexing/ (index integration with storage)
- SPECS/systems/ (I/O, memory, caching, scalability)

## Constraints

- Do not prematurely converge on SQLite, RocksDB, DuckDB, custom binary, etc. until research establishes why
- Research alternatives fairly: investigate multiple approaches per design area
- mmap compatibility required for zero-copy agent retrieval
- Tradeoff analysis: read performance vs write complexity vs memory vs scalability

## Architecture

Five research files covering databases, binary formats, mmap, columnar, and custom storage designs.

## Workflows

- See `.acc/config/workflows/research.md` for conducting storage research.
- See `.acc/config/workflows/feature.md` for adding a new storage system.