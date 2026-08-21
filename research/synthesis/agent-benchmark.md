# Agent Benchmark Synthesis

**Confidence:** OBSERVATION (existing benchmarks), HYPOTHESIS (Prime methodology)
**Primary Sources:** Prime benchmarks, LLM agent evaluations, code intelligence metrics
**Last Updated:** August 2026

## Executive Summary

No existing benchmark specifically measures **code intelligence quality for AI agents**. Current benchmarks focus on: (1) parser performance, (2) query latency, (3) LLM reasoning ability. Prime needs a new benchmark category: **Agent Code Intelligence Quality (ACIQ)**.

## Existing Benchmark Categories

### 1. Parser Performance Benchmarks

| Benchmark | Metrics | Relevance to Prime |
|-----------|---------|-------------------|
| Tree-sitter benchmarks | Parse time, memory | ✅ Direct (Prime uses Tree-sitter) |
| ast-grep benchmarks | Pattern match time | ✅ Direct (Prime could use ast-grep) |
| CodeQL extraction time | Database build time | ⚠️ Indirect (different architecture) |

**Prime's Existing Benchmarks (prime-bench/):**
- Parse time per language
- Index build time
- Query latency
- Storage size
- Compression ratios

### 2. Query Latency Benchmarks

| Benchmark | Metrics | Relevance to Prime |
|-----------|---------|-------------------|
| SCIP index query time | Symbol lookup, reference finding | ✅ Direct |
| LSIF query time | Graph traversal | ✅ Direct |
| Joern query time | CPG traversal | ⚠️ Different model |
| OpenGrok search time | Full-text + cross-ref | ✅ Direct |

### 3. LLM Agent Benchmarks

| Benchmark | Metrics | Relevance to Prime |
|-----------|---------|-------------------|
| SWE-bench | Bug fix success rate | ⚠️ Indirect (tests agent, not index) |
| HumanEval | Code generation pass@k | ❌ Not relevant |
| MBPP | Code generation | ❌ Not relevant |
| CodeContests | Competitive programming | ❌ Not relevant |

**Gap:** No benchmark measures how well a **code intelligence index** supports agent reasoning.

## Agent Code Intelligence Quality (ACIQ) Benchmark

### Definition

ACIQ measures how well a code intelligence system enables AI agents to:
1. **Understand** code structure and semantics
2. **Navigate** to relevant code locations
3. **Reason** about relationships and dependencies
4. **Generate** accurate code modifications

### Benchmark Tasks

#### Task Category 1: Symbol Resolution

| Task | Description | Metric |
|------|-------------|--------|
| **Find Definition** | Given symbol name, find its definition | Precision@1, Recall |
| **Find References** | Given definition, find all references | Recall, Precision |
| **Resolve Import** | Given import statement, resolve to definition | Accuracy |
| **Type Lookup** | Given variable, find its type | Accuracy |

**Example:**
```
Query: "Where is AuthService.login defined?"
Expected: { file: "src/auth/service.rs", line: 42, symbol: "AuthService::login" }
Metric: Precision@1 (is the top result correct?)
```

#### Task Category 2: Relationship Traversal

| Task | Description | Metric |
|------|-------------|--------|
| **Find Callers** | Given function, find all callers | Recall, Precision |
| **Find Callees** | Given function, find all callees | Recall, Precision |
| **Dependency Chain** | Given module, find transitive dependencies | Recall, Precision |
| **Override Chain** | Given method, find all overrides | Recall, Precision |

**Example:**
```
Query: "What functions call AuthService.login?"
Expected: [handle_login, process_request, authenticate_user, ...]
Metric: Recall (did we find all callers?)
```

#### Task Category 3: Impact Analysis

| Task | Description | Metric |
|------|-------------|--------|
| **Change Impact** | Given entity change, list affected entities | Recall, Precision |
| **Blast Radius** | Given change, estimate scope of impact | Accuracy |
| **Test Selection** | Given change, select relevant tests | Recall, F1 |

**Example:**
```
Query: "If I change AuthService.login signature, what breaks?"
Expected: [handle_login, process_request, test_auth, ...]
Metric: Recall (did we catch all affected code?)
```

#### Task Category 4: Context Retrieval

| Task | Description | Metric |
|------|-------------|--------|
| **Relevant Context** | Given question, retrieve relevant code | NDCG, MRR |
| **Token Efficiency** | Given budget, maximize relevant info | Information density |
| **Progressive Disclosure** | Given depth levels, retrieve appropriately | Completeness per level |

**Example:**
```
Query: "How does authentication work?"
Budget: 1000 tokens
Expected: Core auth functions + their relationships (not entire codebase)
Metric: Information density (relevant entities / tokens used)
```

### Benchmark Dataset

#### Synthetic Dataset (Generated)

| Component | Size | Purpose |
|-----------|------|---------|
| Simple project | 1K LOC | Basic symbol resolution |
| Medium project | 10K LOC | Relationship traversal |
| Large project | 100K LOC | Impact analysis |
| Polyglot project | 10K LOC × 5 langs | Cross-language resolution |

#### Real-World Dataset (Curated)

| Repository | Size | Characteristics |
|------------|------|-----------------|
| ripgrep | ~50K LOC | Rust, well-structured |
| bat | ~30K LOC | Rust, good test coverage |
| tree-sitter | ~100K LOC | C, multi-language parsers |
| Code-Graph-RAG | ~20K LOC | Python, multi-language |

### Evaluation Metrics

#### Precision Metrics

| Metric | Definition | Target |
|--------|------------|--------|
| Precision@1 | Top result is correct | >90% |
| Precision@5 | At least 1 of top 5 is correct | >95% |
| Precision@10 | At least 1 of top 10 is correct | >99% |

#### Recall Metrics

| Metric | Definition | Target |
|--------|------------|--------|
| Recall@10 | % of correct results in top 10 | >80% |
| Recall@100 | % of correct results in top 100 | >95% |
| Full Recall | % of correct results found | >99% |

#### Efficiency Metrics

| Metric | Definition | Target |
|--------|------------|--------|
| Latency (p50) | Median query time | <50ms |
| Latency (p99) | 99th percentile query time | <200ms |
| Token efficiency | Relevant entities / tokens | >50% |
| Memory efficiency | Query memory / codebase size | <10% |

### Benchmark Execution

#### Hardware Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | 4 cores | 8+ cores |
| RAM | 8GB | 16GB+ |
| Storage | SSD | NVMe SSD |
| Network | Not required | Not required |

#### Execution Flow

```
1. Build index for benchmark dataset
2. Load index into memory (or mmap)
3. Execute benchmark tasks:
   a. Symbol Resolution: 1000 queries
   b. Relationship Traversal: 500 queries
   c. Impact Analysis: 100 queries
   d. Context Retrieval: 500 queries
4. Collect metrics:
   a. Precision/Recall per task
   b. Latency per query
   c. Memory usage
   d. Token efficiency
5. Generate report
```

### Comparison Framework

| System | ACIQ Score | Latency | Memory | Notes |
|--------|-----------|---------|--------|-------|
| Prime (current) | TBD | ~10ms | ~100MB | Baseline |
| SCIP + Sourcegraph | TBD | ~50ms | ~500MB | Production system |
| Code-Graph-RAG | TBD | ~100ms | ~1GB | Memgraph backend |
| Joern | TBD | ~500ms | ~2GB | JVM overhead |
| OpenGrok | TBD | ~20ms | ~200MB | Search-only |

## Implementation Plan

### Phase 1: Basic Benchmark (1-2 weeks)

- [ ] Create benchmark dataset (synthetic + real-world)
- [ ] Implement symbol resolution tasks
- [ ] Implement relationship traversal tasks
- [ ] Collect baseline metrics

### Phase 2: Advanced Benchmark (2-4 weeks)

- [ ] Implement impact analysis tasks
- [ ] Implement context retrieval tasks
- [ ] Add token efficiency metrics
- [ ] Add memory efficiency metrics

### Phase 3: Comparison (1-2 weeks)

- [ ] Run benchmark on SCIP
- [ ] Run benchmark on Code-Graph-RAG
- [ ] Run benchmark on OpenGrok
- [ ] Generate comparison report

## Open Questions

1. **OPEN QUESTION:** How to handle ground truth for real-world datasets? Manual annotation is expensive; can we use test suites as proxy?

2. **OPEN QUESTION:** How to normalize scores across different codebase sizes? Larger codebases have more entities, affecting recall.

3. **OPEN QUESTION:** Should ACIQ include LLM-based evaluation (e.g., "does the retrieved context enable correct code generation") or pure retrieval metrics?

4. **OPEN QUESTION:** How to benchmark incremental indexing? Need to measure quality after partial updates.

## Confidence Assessment

| Claim | Confidence |
|-------|------------|
| No existing ACIQ benchmark exists | **FACT** (comprehensive search) |
| Symbol resolution is measurable | **FACT** (standard IR metrics) |
| Impact analysis recall is measurable | **OBSERVATION** (test selection benchmarks) |
| Token efficiency is relevant for agents | **INFERENCE** (context window constraints) |
| 50ms latency target is achievable | **HYPOTHESIS** (requires benchmarking) |
