Prime Research Project Documentation

This repository (SPECS/) is a research project investigating how software repositories can be transformed into extremely compact, language-agnostic knowledge representations optimized for machine analysis and AI-agent retrieval.

IMPORTANT:
- Do NOT claim that Prime has already been implemented.
- Do NOT describe an invented architecture as fact.
- Clearly distinguish research from the future product.

## Research Objective
Investigate the complete technical problem of representing a software repository as a compact, queryable, agent-oriented knowledge artifact (see RESEARCH.md for full objective).

## Directory Structure
The SPECS/ folder contains the complete research specification with the following structure:

- `README.md` - This file, project overview
- `RESEARCH.md` - Central research document
- `findings/` - Executive summary, key findings, technical findings, open problems, research gaps
- `prior-art/` - Research of existing systems (SCIP, LSIF, CPG/Joern, Tree-sitter, Graph-sitter, agent indexers, comparison tables)
- `code-analysis/` - Research on what can be derived from source code (parsing, AST/CST, symbols, types, references, call graphs, dependency analysis, data flow, control flow, architecture analysis)
- `storage/` - Storage system research (SQLite, RocksDB, LMDB, custom binary, columnar, mmap, binary formats)
- `compression/` - Compression technique research (integer, graph, string, general compression)
- `indexing/` - Index structure research (symbol indexes, graph indexes, search indexes, succinct data structures)
- `retrieval/` - Retrieval pattern research (information retrieval, agent retrieval, context selection, token efficiency)
- `systems/` - Systems research (I/O, memory, caching, concurrency, scalability)
- `languages/` - Language-agnosticism research (universal models, TypeScript/Rust/Python/Go/Java/other languages)
- `incremental/` - Incremental analysis research (incremental analysis, invalidation, snapshots)
- `reusable-tools/` - Reusable open-source tools research (parsers, analyzers, storage, compression, search)
- `benchmarks/` - Benchmark research (existing benchmarks, methodology, datasets)
- `references/` - References research (papers, repositories, specifications, glossary)

## Key Research Constraints (from init-promt.md)
1. Evidence over assumptions - never write "X is faster" without evidence
2. Primary sources first - prefer official documentation, academic papers, original repositories
3. Do not prematurely converge - do not decide on technologies (SQLite, RocksDB, graph DB, etc.) until research establishes why
4. Research alternatives fairly - for every major design area, investigate multiple approaches
5. Do not implement Prime - this is a research repository, not a product implementation

## How to Use This Research
This research repository provides the knowledge required to build Prime later. The correct outcome is not "Prime is implemented" but "We now understand the technical landscape well enough to design Prime intelligently."

The research covers: information theory, program analysis, compression, indexing, storage systems, language agnosticism, agent architecture, attention mechanisms, semantic hashing, P2P, cryptography, and more.

## License
This is a research project - no formal license. Do not claim product implementation. Do not modify ACC (agents-code-context project).