Prime Research Project - SPECS Folder Guide

This document guides you through the SPECS/ folder structure, which contains the complete research specification for the Prime project.

## Overview
The SPECS/ folder contains 68 markdown files organized into 14 directories, exactly matching the structure specified in init-promt.md (lines 829-930). Each directory serves a specific research purpose.

## Directory Guide

### `findings/` (5 files)
- `executive-summary.md` - High-level summary of research findings
- `key-findings.md` - Most important conclusions from the research
- `technical-findings.md` - Detailed technical findings
- `open-problems.md` - Open problems that remain unsolved
- `research-gaps.md` - Research gaps that need future investigation

### `prior-art/` (7 files)
- `scip.md` - Research on SourceCode Intelligence Protocol (sourcegraph/scip)
- `lsif.md` - Research on Language Server Index Format (microsoft/lsif-node)
- `cpg-joern.md` - Research on Code Property Graphs and Joern
- `tree-sitter.md` - Research on Tree-sitter parser generator
- `graph-sitter.md` - Research on Graph-sitter (ast-grep/ast-grep)
- `agent-indexers.md` - Research on agent-oriented code indexing systems
- `comparison.md` - Detailed comparison tables (all major systems compared across dimensions)

### `code-analysis/` (10 files)
- `parsing.md` - What can be derived from source code (files, directories, packages, modules, symbols, declarations, types, functions, methods, classes, interfaces, variables, constants, parameters, imports, exports, references, calls, inheritance, implementations, instantiation, reads, writes, control flow, data flow, dependencies, reverse dependencies, tests, configuration, resources, generated code, build systems, package managers, architecture, runtime relationships, source provenance, version information)
- `ast-cst.md` - AST/CST research
- `symbols.md` - Symbol identity, resolution, metadata, naming conventions, visibility
- `type-analysis.md` - Type systems, declarations, inference, generics, subtyping, unions, intersections, optionals, abstract types, type aliases, ADTs, mutable/immutable, higher-kinded types, erased types, deprecated types, nested types, bottom type, completeness
- `references.md` - Reference tracking, definition/reference asymmetry, confidence, provenance, unresolved references, reference counting, cyclical references, shadowed references, higher-order references, syntactic vs semantic references, indexing structures
- `call-graphs.md` - Call graph construction, types, concurrency calls, recursive calls, tail call optimization, granularity, inter-procedural analysis, uncertainty, slicing, vs dependency graph
- `dependency-analysis.md` - Build-time vs runtime dependencies, transitive dependencies, cycles, soft vs hard dependencies, invalidation, mapping, reason, age, maintenance status, replacement feasibility, update strategy
- `data-flow.md` - Data flow analysis, def-use chains, kill points, paths, taint analysis, constant propagation, inter-procedural analysis, flow-sensitive vs flow-insensitive, flow-dependent vs flow-independent, alias analysis, points-to analysis, precision tradeoffs, agent context
- `control-flow.md` - Control flow graph, basic blocks, reducible/irreducible, loop identification, cyclomatic complexity, entry/exit nodes, exception flow, coroutine/concurrency flow, pruning, vs call graph, query patterns for agents
- `architecture-analysis.md` - Architecture pattern detection, layer detection, component identification, package/module organization, dependency structure matrix, bounded context, service identification, classification, technical debt indicators, evolution patterns, architecture recovery, agent queries supported, representation formats

### `storage/` (5 files)
- `databases.md` - SQLite, RocksDB, LMDB, DuckDB, custom binary, columnar storage comparison (read performance, random access, sequential access, memory usage, file size, write complexity, update complexity, concurrency, portability, mmap compatibility, scalability, agent suitability)
- `binary-formats.md` - Protobuf, FlatBuffers, Cap'n Proto, MessagePack, CBOR, Apache Arrow, custom binary layouts, zero-copy formats, memory-mappable formats (serialization overhead, deserialization overhead, random access, zero-copy access, schema evolution, compression, file size, implementation complexity)
- `mmap.md` - Memory mapping research (mmap, page cache, page faults, sequential vs random reads, SSD behavior, NVMe behavior, filesystem caching, read amplification, memory locality, CPU cache locality, NUMA, prefetching, zero-copy access, I/O vs read/sread tradeoffs, security considerations, Prime mmap design recommendations)
- `columnar.md` - Columnar storage research (columnar storage, column pruning, predicate pushdown, compression in columnar, dictionary encoding, columnar vs row-oriented tradeoffs, columnar for agent knowledge, implementation considerations, custom binary vs columnar for Prime)
- `custom-storage.md` - Custom storage research (purpose-built storage, append-only, immutable snapshots, Merkle trees, adjacency list, CSR/CSC, LSM-tree, B-tree, cached storage, content-addressed storage, sketch-based storage, hybrid strategies for Prime, storage engine requirements)

### `compression/` (4 files)
- `integer-compression.md` - Varints, SIMD-BP128, Stream VByte, PForDelta, Frame-of-Reference, Elias coding (compression ratio, CPU cost, random access, decompression cost, I/O reduction, best use case, selection matrix, recommendations for Prime)
- `graph-compression.md` - Delta encoding, adjacency compression, WebGraph, succinct graphs, compressed sparse representations (codebase graph compression, tradeoffs, specific techniques for codebase graphs)
- `string-compression.md` - Dictionary encoding, string interning, front coding, tries, FSTs, suffix structures, general string compression (zstd, lz4, brotli, gzip, lzma), front coding + dictionary hybrid, Prime recommendations
- `general-compression.md` - zstd, lz4, brotli, gzip, lzma (comparison matrix, recommendations, chunked approach, compression level strategy, integration points, the overarching principle)

### `indexing/` (4 files)
- `symbol-indexes.md` - Symbol lookup, inverted index (symbol → references/definitions), hybrid symbol index, confidence-annotated symbol index, hashed symbol identity, fuzzy symbol search, cross-language support
- `graph-indexes.md` - Adjacency list index, forward star/reverse star, CSR/CSC, Roaring bitmap, eigenvalue/index-free neighborhood, path queries, tradeoffs, reusable components
- `search-indexes.md` - Inverted index, lexical search, structural search, semantic search, vector search, hybrid search, ranking, filtering, faceting, query expansion, query refinement, precision/recall, tradeoffs, reusable components
- `succinct-structures.md` - Succinct trees, succinct graphs, bit vectors, rank/select, compressed bitmaps, Roaring bitmaps, Elias-Fano, minimal perfect hashing, FSTs, wavelet trees, Prime relevance

### `retrieval/` (4 files)
- `information-retrieval.md` - Inverted indexes, lexical search, symbol search, structural search, semantic search, vector search, hybrid search, ranking, filtering, faceting, query expansion, precision/recall, tradeoffs, reusable components
- `agent-retrieval.md` - Agent retrieval patterns (find symbol, find references, find implementations, navigate relationship, explore context, search codebase), context selection strategies, retrieval granularity, token efficiency, minimum information agent needs, retrieval latency, cached retrieval, failed retrieval handling, reachable retrieval, batch retrieval, privacy, reusable components
- `context-selection.md` - Context selection strategies (minimal context, progressive disclosure, surrounding context, importance-based, recency-based), context selection algorithms (degree-based, hybrid, query-type-specific), token budget management, minimum information agent needs, cross-language support, evaluation metrics, reusable components
- `token-efficiency.md` - Token efficiency (maximizing usefulness per agent context token), token cost model, information density, minimum useful representation, retrieval granularity vs token efficiency, structured vs text representation, attention complexity, keyword/keyphrase extraction, progressive token disclosure, token efficiency metrics, reusable components

### `systems/` (5 files)
- `io.md` - I/O research (I/O patterns, bytes read, pages touched, sequential vs random reads, SSD behavior, NVMe behavior, filesystem caching, memory-mapped I/O advantages/disadvantages, Prime I/O design recommendations, benchmark considerations)
- `memory.md` - Memory research (memory usage patterns, memory-mapped artifact, working set size, cache hierarchy effects, memory allocation overhead, fragmentation, Prime memory design recommendations, incremental memory growth, memory pressure signals, reusable components)
- `caching.md` - Caching research (page cache, agent query cache, symbol metadata cache, relationship cache, cache coherence, warm cache strategies, cache size strategies, eviction policies, read-ahead caching, write-back/write-through, caching for incremental analysis, reusable components)
- `concurrency.md` - Concurrency research (concurrent agent queries, mmap concurrent access, read-write concurrency, artifact modification during querying, lock-free data structures, transactional access, content-addressed concurrency, query parallelization, multi-process agent coordination, concurrent incremental indexing, thread safety of knowledge entries, reusable components)
- `scalability.md` - Scalability research (100K files, 1M files, millions of symbols, tens of millions of relationships, monorepos, generated code, vendored dependencies, duplicated code, multi-language repositories, scaling dimensions, scalability targets, storage engine comparison, Prime scalability design recommendations, reusable components)

### `languages/` (7 files)
- `language-agnostic-models.md` - Language-agnostic models research (universal semantic models, cross-language representation, semantic normalization)
- `typescript.md` - TypeScript language analysis (parsing, symbols, types, classes/interfaces, modules/packages, cross-language relationships, dynamic considerations, type confidence, universal vocabulary mapping examples)
- `rust.md` - Rust language analysis (parsing, symbols, types, traits, ownership/borrowing, memory safety, Cargo packages, specific patterns, universal vocabulary mapping examples)
- `python.md` - Python language analysis (parsing, symbols, types, classes/interfaces, dynamic features, modules/packages, cross-language relationships, universal vocabulary mapping examples)
- `go.md` - Go language analysis (parsing, symbols, types, interfaces, concurrency, universal vocabulary mapping examples)
- `java.md` - Java language analysis (parsing, symbols, types, classes/interfaces, access modifiers, universal vocabulary mapping examples)
- `other-languages.md` - Other languages analysis (C/C++, JavaScript/Node.js, CSS/HTML, Ruby, PHP, Swift, Kotlin, C#, Haskell, OCaml, Scala, common challenges, cross-language mapping principles, language capability model, language adapter pattern, reusable components)

### `incremental/` (3 files)
- `incremental-analysis.md` - Incremental analysis research (incremental parsing, incremental indexing, content hashing, Merkle trees, partial recomputation, Prime recomputation rules, reusable components)
- `invalidation.md` - Invalidation research (change detection, invalidation strategies, file-level/symbol-level/relationship-level, propagation direction, confidence decay, incremental indexing, Merkle DAG invalidation, failure handling, reusable components)
- `snapshots.md` - Snapshots research (immutable snapshots, creation, full vs incremental, comparison (root hash/diff), incremental derivation from snapshots, use cases, storage considerations, agent workflow, consistency, reusable components)

### `reusable-tools/` (5 files)
- `parsers.md` - Reusable parsers research (tree-sitter C11 runtime, 50+ grammars, compiler APIs, LSP clients, parser integration design, tree-sitter as primary backend, compiler API integration, LSP client integration, evaluation criteria, reusable components)
- `analyzers.md` - Reusable analyzers research (data flow analyzer reuse, control flow analyzer reuse, type inference analyzer reuse, custom analyzer development, analyzer integration design, reusable components)
- `storage.md` - Reusable storage research (SQLite, RocksDB, LMDB, custom storage, storage interface abstraction, index structure reuse, Prime storage engine requirements, backend selection matrix, reusable components)
- `compression.md` - Reusable compression research (zstd, lz4, brotli, gzip, lzma, selection matrix, recommendations, integration points, the overarching principle)
- `search.md` - Reusable search research (FAISS/Annoy/ScaNN, Elasticsearch/Whoosh, lexical search adaptation, semantic search adaptation, hybrid search re-ranking, search index adaptation, reusable components)

### `benchmarks/` (3 files)
- `existing-benchmarks.md` - Existing benchmarks (parsing, code search, static analysis, graph traversal, indexing, storage, compression, repository retrieval, coding agent benchmarks, methodology principles)
- `benchmark-methodology.md` - Benchmark methodology research (dimensions: language, scale, polyglot, incremental, agent task type; metrics: latency, throughput, precision/recall, token usage, agent task success rate; dataset design; execution protocol; result analysis; tooling considerations)
- `datasets.md` - Benchmark datasets research (small dataset, medium dataset, large dataset, monorepo simulation, polyglot dataset, dataset generation considerations, availability)

### `references/` (4 files)
- `papers.md` - Academic papers research (code graphs/program analysis papers, information retrieval/agent systems papers, storage/compression papers, now cataloging with format: title, authors, year, venue, URL/DOI, problem, methodology, results, limitations, relevance)
- `repositories.md` - GitHub repositories research (SCIP, LSIF-node, Joern, tree-sitter, Graph-sitter, CodeQL, sourcegraph, rust-analyzer, codeql-action, prime-research/prime, license assessment, maturity, maintenance status, Prime reusable component identification)
- `specifications.md` - Technical specifications research (SCIP protobuf schema, LSIF vertex/edge spec, Tree-sitter CST spec, Joern CPG spec, inverted index spec, vector embedding spec, Merkle tree spec, incremental update spec, confidence/provenance spec, knowledge unit spec, token budget spec, cross-language mapping spec)
- `glossary.md` - Glossary of key terms (to be populated with key terms and definitions used throughout the research)

## Navigation Tips
- Start with `SPECS/README.md` and `SPECS/RESEARCH.md` for project overview
- Use `SPECS/prior-art/comparison.md` for high-level system comparisons
- Browse `SPECS/findings/` for key findings and open problems
- Deep-dive into specific domains using the individual files
- All files maintain consistent formatting: line-numbered content, no inline comments (unless specified), evidence-based content

## Linking Convention
Within documents, references to other SPECS files use the pattern:
- `SPECS/filename.md` (for files in the same directory)
- Or just `filename.md` if the reader is already oriented within SPECS/

Cross-document references should be clear and contextual.

## Maintaining This Repository
- Follow the research principles from init-promt.md
- Keep evidence-based, not AI-generated summaries
- Maintain citations and source links
- Maintain bibliography and glossary
- Keep explicit list of open questions
- When a research area becomes large, split into focused documents (this is why there are many files)
- Do not implement Prime - this is research, not product