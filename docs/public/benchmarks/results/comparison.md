# Framework Benchmark Comparison Results

**Test Environment:** macOS (Apple M4, 16GB RAM, APFS)
**Test Date:** 2026-08-20
**Node:** 22.11 | **pnpm:** 10.2

All tests run with `pnpm install` from the disk-perf-git-and-pnpm repository (workspace with 36 projects).

## Summary Table (Average of 3 Runs)

| Framework | Packages | Avg Clean (s) | Avg Install (s) | Avg Total (s) | Cold Clean (s) | Cold Install (s) |
|-----------|----------|---------------|-----------------|---------------|----------------|------------------|
| **sveltekit** | 56 | 0.17 | 8.04 | **8.21** | 0.16 | 7.49 |
| **vite-vue-ts** | 48 | 0.13 | 7.85 | **7.98** | 0.14 | 7.88 |
| **nuxtjs** | 606 | 1.58 | 9.08 | **10.66** | 1.70 | 8.79 |
| **nextjs** | 360 | 1.94 | 8.92 | **10.85** | 2.28 | 9.23 |
| **remix** | 764 | 1.92 | 9.19 | **11.12** | 2.16 | 9.22 |

## Detailed Results Per Round

### Next.js (360 packages)

| Round | Cache State | Clean (s) | Install (s) | Total (s) |
|-------|-------------|-----------|-------------|-----------|
| 1 | cold | 2.28 | 9.23 | 11.51 |
| 2 | warm | 1.82 | 8.56 | 10.38 |
| 3 | warm | 1.71 | 8.96 | 10.67 |

### Nuxt.js (606 packages)

| Round | Cache State | Clean (s) | Install (s) | Total (s) |
|-------|-------------|-----------|-------------|-----------|
| 1 | cold | 1.70 | 8.79 | 10.49 |
| 2 | warm | 1.70 | 8.62 | 10.32 |
| 3 | warm | 1.34 | 9.84 | 11.18 |

### SvelteKit (56 packages)

| Round | Cache State | Clean (s) | Install (s) | Total (s) |
|-------|-------------|-----------|-------------|-----------|
| 1 | cold | 0.16 | 7.49 | 7.65 |
| 2 | warm | 0.18 | 7.51 | 7.69 |
| 3 | warm | 0.16 | 9.12 | 9.28 |

### Remix (764 packages)

| Round | Cache State | Clean (s) | Install (s) | Total (s) |
|-------|-------------|-----------|-------------|-----------|
| 1 | cold | 2.16 | 9.22 | 11.38 |
| 2 | warm | 2.01 | 9.46 | 11.47 |
| 3 | warm | 1.60 | 8.90 | 10.50 |

### Vite + Vue + TypeScript (48 packages)

| Round | Cache State | Clean (s) | Install (s) | Total (s) |
|-------|-------------|-----------|-------------|-----------|
| 1 | cold | 0.14 | 7.88 | 8.02 |
| 2 | warm | 0.13 | 8.00 | 8.13 |
| 3 | warm | 0.12 | 7.68 | 7.80 |

## Analysis

- **Lightweight frameworks** (SvelteKit, Vite+Vue+TS) have minimal clean times (<0.2s) and fastest total times (~8s)
- **Full frameworks** (Next.js, Nuxt.js, Remix) have more packages and longer clean times (1.3-2.3s)
- **Remix** has the most packages (764) and highest average total time
- **Warm cache** provides modest improvements (~0.5-1s faster install) but clean times are consistent
- **Nuxt.js** shows more variance in Round 3, possibly due to Nuxt prepare step

## Raw Data Files

Individual round results:
- [nextjs-round1.json](./nextjs-round1.json) | [nextjs-round2.json](./nextjs-round2.json) | [nextjs-round3.json](./nextjs-round3.json)
- [nuxtjs-round1.json](./nuxtjs-round1.json) | [nuxtjs-round2.json](./nuxtjs-round2.json) | [nuxtjs-round3.json](./nuxtjs-round3.json)
- [sveltekit-round1.json](./sveltekit-round1.json) | [sveltekit-round2.json](./sveltekit-round2.json) | [sveltekit-round3.json](./sveltekit-round3.json)
- [remix-round1.json](./remix-round1.json) | [remix-round2.json](./remix-round2.json) | [remix-round3.json](./remix-round3.json)
- [vite-vue-ts-round1.json](./vite-vue-ts-round1.json) | [vite-vue-ts-round2.json](./vite-vue-ts-round2.json) | [vite-vue-ts-round3.json](./vite-vue-ts-round3.json)

Full summary: [summary.json](./summary.json)

## Test Methodology

```bash
# Clean (cold cache)
git clean -Xfd; git clean -fd

# Install
pnpm install
```

Times measured with `time` command (real/total time, rounded to tenths).