# OpenGrok — Prior Art Analysis

**Confidence:** OBSERVATION (usage), HYPOTHESIS (performance claims)
**Primary Sources:** OpenGrok documentation, GitHub repository, Oracle engineering blog
**Last Updated:** August 2026

## Executive Summary

OpenGrok is a source code search and cross-reference engine developed by Oracle. It provides fast full-text search and code navigation (definitions, references, call graphs) for large codebases.

## Core Architecture

### Indexing

OpenGrok uses Lucene/Solr for full-text indexing:

| Component | What it captures |
|-----------|------------------|
| **Full-text index** | All source code tokens |
| **Xref database** | Cross-references (definitions, references) |
| **History index** | Git/CVS/SVN history |
| **Project index** | Multi-project support |

### Search Features

| Feature | Description |
|---------|-------------|
| **Full-text search** | Fast keyword search across all files |
| **Symbol search** | Find definitions/references of symbols |
| **Path search** | Search by file path patterns |
| **History search** | Search by commit messages |
| **Cross-reference** | Navigate definitions, references, calls |
| **Call graph** | Who calls/who is called |

## Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| Index build time | 30-300s per 10K LOC | Lucene indexing overhead |
| Index size | 2-5x source code | Lucene index overhead |
| Search latency | 10-100ms | Lucene optimized |
| Memory usage | 200MB-2GB | JVM-based |
| Cross-reference latency | 50-200ms | Xref lookup |

**OBSERVATION:** OpenGrok is optimized for search speed, not semantic analysis.

## Supported Languages

OpenGrok uses Universal Ctags for language parsing, supporting 40+ languages. However, it provides only syntactic analysis (definitions, references), not semantic analysis (types, data flow).

## Comparison with Prime

| Dimension | OpenGrok | Prime |
|-----------|----------|-------|
| **Primary use** | Code search | Agent code intelligence |
| **Search speed** | Very fast (Lucene) | Fast (in-memory) |
| **Analysis depth** | Syntactic only | Semantic |
| **Graph model** | Flat references | Multi-dimensional graph |
| **Agent integration** | HTTP API | MCP server |
| **Incremental updates** | Full reindex | Partial |

## Key Findings for Prime

### ✅ Strengths to Adopt

1. **Lucene-based indexing:** For full-text search, Lucene is hard to beat. Prime could use Lucene or tantivy for keyword search.

2. **Cross-reference database:** OpenGrok's xref database is a lightweight alternative to full semantic analysis.

3. **Multi-project support:** OpenGrok handles large monorepos efficiently.

### ❌ Limitations to Avoid

1. **No semantic analysis:** OpenGrok cannot answer type-related or data-flow questions.

2. **Full reindex required:** No incremental updates.

3. **No agent envelope:** No provenance or token-budget responses.

4. **JVM dependency:** JVM startup time not suitable for interactive agent use.

## Open Questions

- [ ] Can Lucene/tantivy be used for Prime's keyword search alongside graph queries?
- [ ] Is OpenGrok's xref approach sufficient for agent code intelligence?
- [ ] Can Prime use OpenGrok as a search backend for large codebases?

## Confidence Assessment

| Claim | Confidence |
|-------|------------|
| OpenGrok uses Lucene for indexing | **FACT** (architecture documentation) |
| OpenGrok supports 40+ languages | **FACT** (documentation) |
| OpenGrok provides cross-reference navigation | **FACT** (documentation) |
| OpenGrok is syntactic only (no semantic analysis) | **FACT** (limited to ctags-based analysis) |
| Lucene is faster than in-memory for large codebases | **HYPOTHESIS** (requires benchmarking) |
