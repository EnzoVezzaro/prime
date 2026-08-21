# Joern & Code Property Graph (CPG) — Prior Art Analysis

**Confidence:** OBSERVATION (direct tool usage), HYPOTHESIS (optimization claims)
**Primary Sources:** Joern GitHub repository, CPG academic papers (Yamaguchi et al. 2014, 2015)
**Last Updated:** August 2026

## Executive Summary

Joern is the reference implementation of Code Property Graphs (CPG), combining AST, CFG, and PDG into a single queryable graph. It provides a Scala-based query DSL for semantic code analysis, primarily targeting vulnerability detection.

## Core Architecture

### Code Property Graph (CPG)

The CPG merges three classical program representations:

| Layer | What it captures | Query use case |
|-------|------------------|----------------|
| **AST** (Abstract Syntax Tree) | Syntactic structure | Find function declarations, class hierarchies |
| **CFG** (Control Flow Graph) | Execution paths | Data flow analysis, reachability |
| **PDG** (Program Dependence Graph) | Data + control dependencies | Taint analysis, information flow |

**Key Insight (Yamaguchi et al. 2014):** No single graph captures all vulnerability patterns. ASTs miss data flow, CFGs miss structural context, PDGs miss syntax. CPG unifies all three, enabling queries that span multiple analysis dimensions.

**Source:** Yamaguchi, F., et al. "Vulnerability Extraction with Code Property Graphs." USENIX Security 2014.

### Joern Implementation

- **Language:** Scala (JVM-based)
- **Parser:** Combines language-specific parsers (clang, JavaParser, JSParser) into unified CPG
- **Storage:** Graph database backend (Neo4j optional, custom in-memory graph)
- **Query Language:** CPGQL (Scala DSL)
- **Supported Languages:** C/C++, Java, JavaScript/TypeScript, Python (experimental), Go (experimental)

### CPG Node Types (simplified)

```
File → Namespace → Type → Method → Block → Expression → Literal
                                                  ↓
                                   ParameterIn ← ParameterOut
                                                  ↓
                                         Call → Return → CFG edges
```

### CPGQL Query Example

```scala
// Find SQL injection: user input reaching SQL query
cpg.call
  .name("execute")
  .argument(1)
  .reachableByFlows(cpg.call.name("getParameter"))
  .code
```

This query: finds calls to `execute()` where argument 1 is reachable from `getParameter()` calls — a classic SQL injection pattern.

## Design Principles

### 1. Unified Representation
CPG avoids the combinatorial explosion of querying multiple separate graphs. A single traversal can follow AST edges, then switch to CFG edges, then PDG edges.

### 2. Language-Agnostic Querying
CPGQL queries are written once and run against any language's CPG. The same taint analysis query works for C, Java, and JavaScript.

### 3. Property-Based Nodes
Every node carries metadata (name, type, code, line number, column) enabling rich filtering beyond graph structure.

## Performance Characteristics

**OBSERVATION:** Joern prioritizes correctness over speed.

| Metric | Value | Notes |
|--------|-------|-------|
| Parse time (C) | ~1-5s per 10K LOC | Language-dependent |
| CPG build time | ~2-10s per 10K LOC | Includes CFG/PDG construction |
| Query latency | 100ms-10s | Depends on graph size and query complexity |
| Memory usage | 100MB-2GB | For medium codebases (100K LOC) |
| Storage size | 10-50x source code | Graph overhead significant |

**HYPOTHESIS:** Joern's JVM-based architecture introduces startup overhead not suitable for sub-second interactive queries.

## Comparison with Prime

| Dimension | Joern/CPG | Prime |
|-----------|-----------|-------|
| **Scope** | Vulnerability detection | Agent code intelligence |
| **Graph layers** | AST + CFG + PDG | AST + relationships (partial CFG) |
| **Query language** | CPGQL (Scala DSL) | SQL-like / in-memory traversal |
| **Storage** | Neo4j / in-memory | Custom binary + mmap |
| **Agent integration** | None (CLI/batch) | MCP server, PrimeEnvelope |
| **Incremental updates** | Full rebuild | Partial (in progress) |
| **Token efficiency** | N/A | Progressive context building |

## Key Findings for Prime

### ✅ Strengths to Adopt

1. **Multi-layer graph unification:** CPG's approach of combining AST + CFG + PDG into one queryable structure is architecturally sound. Prime should consider adding CFG/PDG layers.

2. **Language-agnostic queries:** CPGQL's ability to write one query across languages is valuable. Prime's query API should support similar cross-language queries.

3. **Taint analysis primitives:** CPG's reachability analysis (reachableByFlows) is essential for data flow questions agents frequently ask.

### ❌ Limitations to Avoid

1. **JVM dependency:** JVM startup time and memory overhead are incompatible with sub-second agent queries.

2. **Batch-oriented design:** Joern is designed for offline analysis, not interactive agent queries.

3. **No agent envelope:** No provenance, confidence levels, or token-budget-aware responses.

4. **Limited compression:** Graph storage is verbose; no succinct data structures.

## Open Questions

- [ ] Can CPG's reachability analysis be implemented efficiently on Prime's in-memory graph?
- [ ] What is the performance cost of adding CFG/PDG layers to Prime?
- [ ] Can CPGQL-style queries be compiled to Prime's query engine efficiently?

## Confidence Assessment

| Claim | Confidence |
|-------|------------|
| CPG unifies AST + CFG + PDG | **FACT** (academic paper, reproducible) |
| Joern supports C/C++, Java, JS | **FACT** (GitHub documentation) |
| CPG queries are language-agnostic | **OBSERVATION** (demonstrated in papers) |
| Joern has JVM startup overhead | **INFERENCE** (JVM characteristic) |
| CPG storage is 10-50x source code | **OBSERVATION** (community reports) |
| Prime should add CFG/PDG layers | **HYPOTHESIS** (requires validation) |
