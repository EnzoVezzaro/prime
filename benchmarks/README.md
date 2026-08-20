# Prime Benchmarks

This directory contains the benchmark infrastructure for Prime.

## Overview

The benchmark system measures whether Prime achieves its core objective: **maximizing the number and quality of agent questions that can be answered without retrieving source code, while minimizing the amount of information, I/O, computation, latency, and context required.**

## Directory Structure

```
benchmarks/
├── corpus/                    # Benchmark corpus definitions
│   ├── repositories.json      # Repository definitions (pinned to immutable commits)
│   └── questions/             # Knowledge question corpus
│       └── knowledge.json     # Deterministic questions with expected answers
├── schemas/
│   └── prime-benchmark-result.schema.json  # JSON Schema for result validation
├── results/
│   └── latest.json            # Latest benchmark result (generated, committed)
├── scripts/
│   ├── run-benchmark.sh       # Main benchmark runner
│   ├── validate-result.py     # Validates result against schema
│   └── update-readme.py       # Updates README.md with benchmark results
├── README.md                  # This file
```

## Running Benchmarks

### Quick PR Benchmark
```bash
./benchmarks/scripts/run-benchmark.sh --corpus pr
```

### Full Nightly Benchmark
```bash
./benchmarks/scripts/run-benchmark.sh --corpus nightly
```

### Custom Corpus
```bash
./benchmarks/scripts/run-benchmark.sh --corpus /path/to/repos
```

### Manual Run
```bash
cargo run --release --bin prime -- benchmark --corpus pr --output benchmarks/results/latest.json
```

## Corpus

### PR Corpus (Fast, for CI)
Small, fast-to-analyze repositories:
- `bat` (Rust, small)
- `httpx` (Python, small)
- `express` (JavaScript, small)
- `gin` (Go, small)
- `spdlog` (C++, small)

### Nightly Corpus (Complete)
All repositories across 10+ languages and sizes from small to large.

## Metrics Measured

### Derivation
- `time_ms` - Total time to build knowledge graph
- `files_per_second` - Throughput
- `symbols_per_second` - Entity extraction rate
- `relationships_per_second` - Relation extraction rate

### Artifact
- `source_bytes` - Total source size
- `artifact_bytes` - Prime knowledge graph size
- `derived_representation_ratio` - Artifact / source ratio
- `artifact_bytes_per_kloc` - Size efficiency

### Retrieval (in microseconds)
- `cold` - First query after load
- `warm` - Subsequent queries
- `search` - Keyword search latency
- `lookup` - Exact symbol lookup
- `context` - Context assembly

### Knowledge (Source-Free Answer Rate)
- `total_questions` - Number of questions asked
- `answered_without_source` - Questions answered from Prime alone
- `source_free_answer_rate` - Ratio
- `by_category` - Per-category breakdown

### Source Savings
- `reduction_ratio` - How much source retrieval is avoided

## Result Format

Results are written to `benchmarks/results/latest.json` following the schema in `schemas/prime-benchmark-result.schema.json`.

The result includes:
- Schema version
- Benchmark metadata (commit, timestamp, dirty flag)
- Prime version info
- Environment (OS, CPU, memory, Rust version)
- Per-repository corpus info
- Per-repository benchmark results
- BMF-compatible metrics for Bencher integration
- Overall status

## Validation

```bash
python3 scripts/validate-result.py benchmarks/results/latest.json
```

## CI Integration

### PR Benchmark (`.github/workflows/benchmark.yml`)
Runs on every PR and push to main. Posts results as PR comment.

### Nightly Benchmark (`.github/workflows/benchmark-nightly.yml`)
Runs daily at 3 AM UTC with full corpus. Optional Bencher upload.

## BMF Compatibility

The benchmark produces Bencher Metric Format (BMF) compatible metrics for historical tracking and regression detection.

```json
{
  "prime::derivation": { "latency": { "value": 688.0, "unit": "ms" } },
  "prime::artifact": { "file-size": { "value": 1438505, "unit": "bytes" } },
  "prime::retrieval::warm": { "latency": { "value": 33.0, "unit": "us" } },
  "prime::knowledge::source_free_rate": { "ratio": { "value": 0.0 } }
}
```

## Reproducibility

To reproduce a benchmark result:

```bash
git clone https://github.com/EnzoVezzaro/prime
cd prime
git checkout <commit-from-result>
cargo build --release --workspace --manifest-path prime-rs/Cargo.toml
./benchmarks/scripts/run-benchmark.sh --corpus pr
```

## Result Policy

- `latest.json` is committed to the repository
- Only the normalized result is committed (no raw logs, no artifacts)
- Historical releases can create `results/releases/vX.Y.Z.json`
- Time-series tracking delegated to Bencher

## Adding a Repository

1. Add entry to `corpus/repositories.json` with pinned commit
2. Clone to `prime-rs/benchmarks/repos/<name>`
3. Run benchmark to verify

## Adding Knowledge Questions

1. Add to `corpus/questions/knowledge.json`
2. Each question must have: ID, category, question text, expected knowledge, source_allowed flag
3. Questions should be answerable from derived knowledge (symbols, relationships, types)

## Limitations

- Current implementation is single-threaded per repository
- Knowledge evaluation uses simple keyword matching (not semantic)
- Source-free answer rate is a proxy metric
- Source savings measurement is not yet implemented
- Requires pre-cloned repositories

## Future Work

- [ ] Semantic knowledge evaluation (not just keyword)
- [ ] Source savings measurement with baseline agent
- [ ] Agent efficiency metrics (tool calls, tokens, etc.)
- [ ] Distributed benchmark execution
- [ ] Learn-to-rank for retrieval quality
- [ ] Larger corpus for scale testing