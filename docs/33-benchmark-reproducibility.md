---
title: Benchmark Reproducibility
---

# Benchmark Reproducibility

## Running Benchmarks

```bash
# From prime-rs directory
cargo bench -p prime-bench

# Specific benchmark
cargo bench -p prime-bench --bench benchmarks -- bench_parsing

# With custom corpus
PRIME_BENCH_CORPUS=/path/to/repo cargo bench -p prime-bench
```

## Environment

Pinned for reproducibility:

| Component | Version |
|-----------|---------|
| Rust | 1.79+ (MSRV) |
| Tree-sitter | 0.25 |
| Criterion | 0.5 |
| OS | macOS / Linux |
| CPU | Documented per run |

## Corpus

Repositories pinned to specific commits:

| Repo | Language | Commit | Size |
|------|----------|--------|------|
| bat | Rust | `abc123...` | ~50 files |
| httpx | Python | `def456...` | ~80 files |
| express | JavaScript | `ghi789...` | ~120 files |
| gin | Go | `jkl012...` | ~200 files |
| spdlog | C++ | `mno345...` | ~300 files |

## Output Format

Machine-readable JSON written to `benchmarks/results/latest.json`:

```json
{
  "timestamp": "2026-08-20T...",
  "environment": { "os": "macos", "arch": "aarch64", "cpu": "Apple M2" },
  "repos": [
    { "name": "bat", "language": "rust", "commit": "abc123...", "metrics": { ... } }
  ],
  "aggregate": { "entity_f1": 0.37, "retrieval_p50_us": 169, ... }
}
```

## CI Integration

Benchmarks run on every PR:
- Regression detection (p95 latency, accuracy)
- Artifact size monitoring
- Derivation time tracking

## Custom Corpora

Add your own repository:

```bash
# Single repo
cargo run --bin prime -- build --root /path/to/repo --storage /tmp/prime-bench
cargo bench -p prime-bench -- bench_queries

# Multiple repos
PRIME_BENCH_CORPUS=/path/to/corpus cargo bench -p prime-bench
```

Corpus structure:
```
corpus/
├── repo1/
│   └── .prime-bench-commit  # contains pinned commit hash
├── repo2/
│   └── .prime-bench-commit
```

## Comparing Runs

```bash
# Compare two result files
cargo run --bin prime-bench -- compare results/v0.1.0.json results/latest.json
```

Outputs:
- Metric deltas
- Regression alerts
- Improvement highlights

## Next

- [Benchmark Methodology](./benchmark-methodology.md)
- [Benchmark Results](./benchmark-results.md)