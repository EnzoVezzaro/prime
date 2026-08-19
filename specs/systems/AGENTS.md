# systems

## Purpose

Research on system-level concerns for codebase knowledge artifacts: I/O, memory, caching, concurrency, scalability.

## Responsibilities

- Research I/O patterns (bytes read, pages touched, mmap, SSD/NVMe, sequential vs random)
- Research memory patterns (working set, cache hierarchy, mmap, fragmentation, NUMA)
- Research caching strategies (page cache, agent query cache, symbol/relationship caches)
- Research concurrency (mmap concurrent access, lock-free structures, incremental updates)
- Research scalability (100K-1M+ files, millions of symbols, monorepos, multi-language)

## Ownership

Owner: research team

## Inputs

- OS/storage documentation (mmap, page cache, NVMe)
- Database concurrency models (LSM vs B-tree, readers-writer locks)
- Cache algorithms (LRU, LFU, read-ahead)
- Scalability benchmarks (100K to millions of entities)

## Outputs

- SPECS/systems/io.md
- SPECS/systems/memory.md
- SPECS/systems/caching.md
- SPECS/systems/concurrency.md
- SPECS/systems/scalability.md

## Dependencies

- SPECS/storage/ (storage engine I/O patterns)
- SPECS/compression/ (compression I/O patterns)
- SPECS/indexing/ (index access patterns)

## Constraints

- Prime's performance measured in: bytes read, pages touched, allocations, CPU cycles, latency (not just "query speed")
- Must support partial loading and partial retrieval (5M-entity monorepo should not require loading entire artifact)
- Must support incremental derivation and incremental invalidation
- Must scale across three dimensions: files, entities, relationships

## Architecture

Five research files covering I/O, memory, caching, concurrency, scalability. Cross-cutting concerns that affect all other research areas.

## Workflows

- See `.acc/config/workflows/research.md` for conducting systems research.
- See `.acc/config/workflows/feature.md` for adding a new systems area.