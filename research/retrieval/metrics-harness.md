# Metrics Harness for Retrieval Precision

## Metrics
precision@k, recall@k, nDCG@k, MRR
entity precision/recall/F1
relationship precision/recall/F1
candidate_result_ratio
evidence_density
bytes_retrieved, tokens_retrieved

## Ground Truth
Need repo-specific benchmark questions with verified answers.

## Harness API
run_benchmark(repo, queries) -> MetricsReport
compare_strategies(A,B) -> delta

## Implementation
Create prime-benchmarks crate with harness.
Integrate with telemetry from QueryEngine.

## Reporting
JSON report with per-query and aggregate metrics.
