# CodeQL — Prior Art Analysis

**Confidence:** OBSERVATION (usage), HYPOTHESIS (performance claims)
**Primary Sources:** GitHub CodeQL documentation, CodeQL CLI repository, CodeQL paper (2020)
**Last Updated:** August 2026

## Executive Summary

CodeQL is GitHub's semantic code analysis engine, treating code as queryable data. It uses a custom query language (QL, based on Datalog) to express complex code patterns including vulnerabilities, code smells, and security anti-patterns.

## Core Architecture

### Query Language (QL)

CodeQL's QL is a declarative, Datalog-based language designed for recursive queries over code structure:

```ql
// Find path traversal vulnerabilities
from MethodAccess ma, Method m
where ma.getMethod() = m
  and m.hasName("File")
  and m.getDeclaringType().hasQualifiedName("java.io", "File")
  and ma.getAnArgument().getStringValue().matches("%..%")
select ma, "Potential path traversal"
```

**Key Design Principle:** CodeQL compiles queries to Datalog, which is then evaluated over a pre-built database. This separates "what to query" from "how to query."

### CodeQL Database

A CodeQL database is a structured representation of code:

| Component | What it captures |
|-----------|------------------|
| **AST** | Complete syntax tree |
| **Name binding** | Resolved references, scopes |
| **Type information** | Full type hierarchy |
| **Control flow** | Basic blocks, dominance |
| **Data flow** | SSA form, def-use chains |
| **Type flow** | Type inference, substitution |

**Storage Format:** Proprietary, optimized for QL evaluation. Not designed for external consumption.

### Supported Languages

| Language | Support Level | Notes |
|----------|---------------|-------|
| C/C++ | Mature | Clang-based extraction |
| Java | Mature | javac-based extraction |
| C# | Mature | Roslyn-based extraction |
| JavaScript | Mature | Tree-sitter based |
| Python | Mature | Custom extractor |
| Go | Mature | go/parser based |
| Ruby | Growing | Community + GitHub |
| Swift | Growing | Apple contribution |
| Kotlin | Experimental | Via Java extractor |
| Rust | Experimental | Community work |

## Query Execution Model

### Phase 1: Extraction
Language-specific extractors parse source code and build a CodeQL database (BQRS format).

### Phase 2: Evaluation
QL queries are compiled to Datalog and evaluated using a custom solver.

### Phase 3: Materialization
Results are materialized into tables (CSV or BQRS format).

**OBSERVATION:** The extraction phase is language-specific but query evaluation is language-agnostic.

## Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| Database build time | 10-60s per 10K LOC | Language-dependent |
| Database size | 5-20x source code | Proprietary format |
| Query compilation | 1-10s | Datalog compilation overhead |
| Query evaluation | 100ms-60s | Depends on codebase size |
| Memory usage | 500MB-4GB | For medium codebases |

**HYPOTHESIS:** CodeQL's compilation step adds latency not suitable for interactive agent queries.

## Comparison with Prime

| Dimension | CodeQL | Prime |
|-----------|--------|-------|
| **Primary use** | Security analysis, code review | Agent code intelligence |
| **Query language** | QL (Datalog-based) | API-based, in-memory |
| **Extraction** | Language-specific extractors | Tree-sitter unified |
| **Storage** | Proprietary BQRS | Custom binary + mmap |
| **Graph model** | Multi-layer (AST, data flow, control flow) | Single-layer (entities + relations) |
| **Agent integration** | GitHub Actions only | MCP server |
| **Open source** | QL language yes, extractor parts | Fully open source |

## Key Findings for Prime

### ✅ Strengths to Adopt

1. **Separation of extraction and query:** CodeQL cleanly separates parsing/extraction from query evaluation. Prime should maintain this separation.

2. **Multi-layer analysis:** CodeQL's ability to query across AST, data flow, and type flow simultaneously is powerful.

3. **Recursive query support:** Datalog-based queries can express complex recursive patterns (transitive dependencies, reachability).

4. **Language-agnostic queries:** Once extracted, the same QL query works across all supported languages.

### ❌ Limitations to Avoid

1. **Extraction overhead:** Building a CodeQL database is slow (minutes for large codebases).

2. **Proprietary storage:** BQRS format is not designed for agent consumption.

3. **No real-time updates:** Full extraction required for each change.

4. **Complex query syntax:** QL is powerful but has a steep learning curve for agents.

5. **No agent envelope:** No provenance, confidence, or token-budget responses.

## CodeQL vs. Semgrep

| Dimension | CodeQL | Semgrep |
|-----------|--------|---------|
| **Approach** | Full semantic extraction | Pattern matching + semantic |
| **Query power** | Maximum (Datalog) | Limited (pattern-based) |
| **Speed** | Slow (compilation) | Fast (pattern matching) |
| **Ease of use** | Complex | Simple |
| **Custom rules** | Full QL expressions | Pattern + taint modes |
| **Best for** | Deep semantic analysis | Quick pattern detection |

**OBSERVATION:** Prime could benefit from both approaches: CodeQL-style deep analysis for complex queries, Semgrep-style pattern matching for simple lookups.

## Open Questions

- [ ] Can CodeQL's Datalog queries be translated to Prime's query engine?
- [ ] Is CodeQL's extraction speed acceptable for incremental agent updates?
- [ ] Can Prime use CodeQL as a backend for deep semantic queries?
- [ ] How does CodeQL's type flow analysis compare to Tree-sitter's type inference?

## Confidence Assessment

| Claim | Confidence |
|-------|------------|
| CodeQL uses Datalog-based queries | **FACT** (official documentation) |
| CodeQL supports 10+ languages | **FACT** (GitHub documentation) |
| Database build is language-specific | **FACT** (architecture documentation) |
| CodeQL compilation adds latency | **INFERENCE** (Datalog characteristic) |
| CodeQL is not suitable for interactive use | **HYPOTHESIS** (requires benchmarking) |
| Prime should add Datalog query support | **OPEN QUESTION** |
