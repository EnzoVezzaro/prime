# reusable-tools

## Purpose

Research on reusable open-source tools for parsing, analysis, storage, compression, and search.

## Responsibilities

- Research and catalog parser libraries (Tree-sitter, compiler APIs, LSP clients)
- Research and catalog analyzer frameworks (data flow, control flow, type inference)
- Research and catalog storage engines (SQLite, RocksDB, LMDB, custom binary)
- Research and catalog compression libraries (zstd, lz4, SIMD compression)
- Research and catalog search libraries (FAISS, Annoy, ScaNN, Elasticsearch)

## Ownership

Owner: research team

## Inputs

- GitHub repositories for all candidate tools
- Official documentation and benchmarks
- License information (Apache-2.0, MIT, GPL, etc.)
- Maturity and maintenance status assessment

## Outputs

- SPECS/reusable-tools/parsers.md
- SPECS/reusable-tools/analyzers.md
- SPECS/reusable-tools/storage.md
- SPECS/reusable-tools/compression.md
- SPECS/reusable-tools/search.md

## Dependencies

- SPECS/prior-art/ (for tools used by prior art systems)
- SPECS/storage/ (storage engine evaluation)
- SPECS/compression/ (compression library evaluation)
- SPECS/indexing/ (search index evaluation)

## Constraints

- Never recommend a dependency without checking its actual repository and license
- Categorize each: REUSE, ADAPT, STUDY, REPLACE, AVOID
- Check actual repository and license, not just marketing
- Performance characteristics must be documented

## Architecture

Five research files: parsers, analyzers, storage, compression, search. Each categorizes tools as REUSE/ADAPT/STUDY/REPLACE/AVOID.

## Workflows

- See `.acc/config/workflows/research.md` for conducting tool research.
- See `.acc/config/workflows/feature.md` for adding a new tool category.