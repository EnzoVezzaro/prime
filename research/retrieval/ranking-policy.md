# Ranking Policy per Intent

## Purpose
Map RetrievalIntent to deterministic scoring weights for signals.

## Signals
name_exact_match
name_prefix_match
name_contains
qualified_ends_with
qualified_contains
doc_contains
confidence

## Policies
Lookup:
name_exact_match 100
qualified_ends_with 90
name_prefix_match 80
...

Search:
name_exact_match 100
name_prefix_match 85
qualified_ends_with 60
qualified_contains 40
doc_contains 10

Callers:
Prefer function/method kinds, relationship existence.

## Implementation
RankingPolicy struct with weights HashMap<Signal, f32>
select_policy(intent) -> RankingPolicy
score = Σ weight * normalized_signal

## Telemetry
Add signal breakdown per top candidate.
