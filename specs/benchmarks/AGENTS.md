# benchmarks

## Purpose

Research on benchmarking methodology, existing benchmarks, and datasets for Prime validation.

## Responsibilities

- Research existing benchmarks (parsing, code search, static analysis, graph traversal, indexing, storage, compression, agent retrieval)
- Design Prime benchmark methodology (dimensions, metrics, datasets)
- Catalog benchmark datasets (small, medium, large, monorepo, polyglot)

## Ownership

Owner: research team

## Inputs

- Existing benchmark suites (CodeSearchNet, CodeXGLUE, HumanEval, MBPP, etc.)
- Codebase datasets (GitHub Code, BigQuery public datasets, synthetic benchmarks)
- Academic papers on benchmark methodology
- ACC benchmark infrastructure

## Outputs

- SPECS/benchmarks/existing-benchmarks.md
- SPECS/benchmarks/benchmark-methodology.md
- SPECS/benchmarks/datasets.md

## Dependencies

- SPECS/systems/ (scalability targets)
- SPECS/compression/ (compression benchmarks)
- SPECS/indexing/ (indexing benchmarks)

## Constraints

- Do not run large experiments yet unless useful for validating a research question
- Design benchmark dimensions: language, scale, polyglot, incremental, agent task type
- Metrics: latency, throughput, precision@k, recall@k, token usage, agent task success rate
- Test architecture against small, large, and polyglot repositories from the beginning

## Architecture

Three research files: existing benchmarks, methodology, datasets. Cross-referenced from all validation work.

## Workflows

- See `.acc/config/workflows/research.md` for conducting benchmark research.
- See `.acc/config/workflows/feature.md` for adding a new benchmark.