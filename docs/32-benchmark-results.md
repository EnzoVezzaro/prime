---
title: Benchmark Results
---

# Benchmark Results

Latest results loaded from [`benchmarks/results/latest.json`](../benchmarks/results/latest.json).

## Current Metrics

| Metric | Result |
|--------|-------:|
| Derivation | 334 ms |
| Artifact size | 1.2 MB |
| Artifact/Source ratio | 1.196 |
| Retrieval p50 (warm) | 169 µs |
| Retrieval p95 (warm) | 305 µs |
| Accuracy | 8.2% |
| Source-free accuracy | 8.2% |
| Entity precision | 0.35 |
| Entity recall | 0.40 |
| Entity F1 | 0.37 |
| Relationship precision | 0.00 |
| Relationship recall | 0.00 |
| Relationship F1 | 0.00 |
| MRR | 0.40 |
| Recall@1 | 0.1% |
| Recall@3 | 0.1% |
| Recall@5 | 0.1% |
| Recall@10 | 0.1% |

**Repositories tested:** `bat` (Rust), `httpx` (Python), `express` (JavaScript), `gin` (Go), `spdlog` (C++)
**Environment:** macOS / aarch64 / Apple M2
**Integrity:** ✅ Valid
**Repos:** 5/5 completed

## Framework Installation Benchmarks

We also tested 5 popular JavaScript/TypeScript frameworks using the [disk-perf-git-and-pnpm](https://github.com/NullVoxPopuli/disk-perf-git-and-pnpm) methodology.

See full comparison: [`comparison.md`](../benchmarks/results/comparison.md)

**Quick Summary (3-run averages):**

| Framework | Packages | Avg Total (s) | Cold Install (s) |
|-----------|----------|---------------|------------------|
| Vite + Vue + TS | 48 | **7.98** | 7.88 |
| SvelteKit | 56 | **8.21** | 7.49 |
| Nuxt.js | 606 | **10.66** | 8.79 |
| Next.js | 360 | **10.85** | 9.23 |
| Remix | 764 | **11.12** | 9.22 |

Lightweight frameworks (Vite+Vue, SvelteKit) install ~3x faster than full frameworks (Remix, Next.js) primarily due to fewer packages.

## Interpretation

**Current accuracy is low (8.2%)** — this is expected for research-stage. The current implementation:
- Extracts symbols and basic relationships
- Does not yet implement full semantic analysis
- Relationship extraction needs significant improvement

**Retrieval is fast** — sub-millisecond p50 latency demonstrates the architecture can deliver on speed.

**Artifact size is reasonable** — 1.196x source size includes full indexing structures.

## Next Steps for Accuracy

1. Complete type inference and resolution
2. Implement cross-file relationship analysis
3. Add control flow and data flow analysis
4. Improve symbol identity across languages
5. Validate against ground-truth agent tasks

## Historical Results

Results are versioned in `benchmarks/results/`:
- `latest.json` — current run
- `v0.1.0.json` — initial baseline
- `v0.2.0.json` — after incremental indexing

Run benchmarks yourself:
```bash
cd prime-rs
cargo bench -p prime-bench
```

## Next

- [Benchmark Methodology](./benchmark-methodology.md)
- [Reproducibility](./benchmark-reproducibility.md)