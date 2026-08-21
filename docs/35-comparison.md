---
title: Comparison
---

# Comparison

Prime compared to related systems.

## Code Intelligence Systems

| System | Focus | Prime Difference |
|--------|-------|------------------|
| **SCIP** | Cross-language indexing protocol | Prime is agent-optimized representation, not protocol |
| **LSIF** | Persistent language server data | Prime is semantic, not LS protocol dump |
| **CPG/Joern** | Rich program graphs (AST+CFG+PDG) | Prime is minimal agent-useful knowledge |
| **Tree-sitter** | Incremental parsing | Prime uses Tree-sitter as frontend |
| **Sourcegraph** | Code search + AI | Prime is local, offline, artifact-based |
| **Aider repo map** | Compact structural map for LLMs | Prime is structured, typed, queryable |
| **Graph-sitter** | Graph-based code analysis | Prime targets agent retrieval, not analysis |

## Storage & Indexing

| System | Focus | Prime Difference |
|--------|-------|------------------|
| **SQLite** | Relational storage | Prime may use any storage; not tied to SQL |
| **RocksDB** | LSM-tree key-value | Prime evaluates alternatives fairly |
| **LMDB** | Memory-mapped B+tree | Prime's mmap is OS-managed, format-agnostic |
| **Custom binary** | Prime's current approach | Research-driven, not premature |
| **Columnar (Parquet)** | Analytics | Prime optimizes for point queries |
| **Graph DB (Neo4j)** | Graph traversal | Prime's relations are in-artifact |

## Compression

| Technique | Prime Relevance |
|-----------|-----------------|
| **Grammar compression** | Research: pattern deduplication |
| **Delta encoding** | Research: version diffs |
| **Succinct structures** | Research: rank/select, bitmaps |
| **Learned indexes** | Research: replace B-trees |
| **WebGraph** | Research: graph compression |

## Agent Systems

| System | Focus | Prime Difference |
|--------|-------|------------------|
| **LangChain** | LLM orchestration | Prime is knowledge layer, not orchestration |
| **AutoGPT** | Autonomous agents | Prime is what agents consume |
| **Cursor/Copilot** | IDE-integrated agents | Prime is repository-native, IDE-agnostic |
| **Aider** | CLI agent | Prime provides the knowledge Aider maps approximate |

## What Prime Is Not

| Category | Examples |
|----------|----------|
| Database | Not SQLite, RocksDB, Neo4j, etc. |
| Compiler | Not rustc, tsc, javac |
| Language server | Not rust-analyzer, tsserver |
| Documentation generator | Not Sphinx, JSDoc, rustdoc |
| Vector store | Not Pinecone, Weaviate, Qdrant |
| Search engine | Not Elasticsearch, Meilisearch |
| CI/CD | Not GitHub Actions, GitLab CI |
| Package manager | Not npm, Cargo, pip |

## Prime's Unique Position

```
Prime = Derived Knowledge Layer
         │
         ├── Optimized for AGENT QUESTIONS
         ├── SEMANTIC TOOLS (not graph primitives)
         ├── PROVENANCE + CONFIDENCE (first-class)
         ├── SOURCE ESCALATION (not elimination)
         ├── LANGUAGE AGNOSTIC (universal model)
         ├── MCP NATIVE (agent interface)
         └── RESEARCH FIRST (evidence over assumptions)
```