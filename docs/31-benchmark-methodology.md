---
title: Benchmark Methodology
---

# Benchmark Methodology

Prime benchmarks measure what matters for agent codebase understanding.

## Corpus

- Pinned commits for reproducibility
- Diverse languages: TypeScript, Python, Rust, Go, Java, C++, C
- Diverse sizes: tiny (10 files) → large (100K+ files)
- Real-world repositories (not synthetic)

## Benchmark Categories

### Derivation Benchmarks
- **Parse time**: Time to parse repository
- **Analysis time**: Symbol extraction, relationship derivation
- **Incremental update**: Time to update after N file changes
- **Language coverage**: % of symbols/types/relations extracted per language

### Artifact Benchmarks
- **Artifact size**: Bytes on disk
- **Compression ratio**: Artifact size / source size
- **Serialization time**: Write artifact to disk
- **Deserialization time**: Load artifact from disk
- **Memory footprint**: RSS when loaded

### Retrieval Benchmarks
- **p50 latency**: Median query time (warm)
- **p95 latency**: 95th percentile query time
- **p99 latency**: 99th percentile query time
- **Throughput**: Queries per second
- **Cold start**: First query after load

### Knowledge Benchmarks
- **Entity precision**: % of returned entities that are correct
- **Entity recall**: % of true entities returned
- **Entity F1**: Harmonic mean
- **Relationship precision/recall/F1**: Same for relations
- **Source-free answer rate**: % of questions answered without source access
- **MRR**: Mean Reciprocal Rank for search
- **Recall@K**: For search (K=1,3,5,10)

### Agent Benchmarks
- **Task success rate**: % of agent tasks completed correctly
- **Tool calls**: Number of Prime tool calls per task
- **Tokens transferred**: Total tokens in tool results
- **Source reads avoided**: Source files NOT read due to Prime
- **Corrections needed**: Agent self-corrections due to Prime errors

### Scalability Benchmarks
- **Memory scaling**: RSS vs repository size
- **Time scaling**: Derivation time vs file count
- **Parallel efficiency**: Speedup with N cores
- **Sharding overhead**: Cost of distributed analysis

## Statistical Methodology

- Minimum 10 runs per benchmark
- Report median, p50, p95, p99
- Warm/cold separation
- CI/CD integration for regression detection
- Hardware specification pinned

## Reproducibility

All benchmarks:
- Pinned dependencies
- Pinned repository commits
- Deterministic seeds
- Machine-readable output: `benchmarks/results/latest.json`
- Reproducible via `cargo bench -p prime-bench`

## Next

- [Benchmark Results](./benchmark-results.md)
- [Reproducibility](./benchmark-reproducibility.md)