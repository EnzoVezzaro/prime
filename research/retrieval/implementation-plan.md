# Retrieval Precision Experiment – Implementation Plan

## Constraints
Parser and Knowledge Graph frozen. No new relationships/entities. No latency optimization. No embeddings.

## Phase 1 – Explain Mode
Add `prime query --explain` CLI flag.
Extend QueryEngine to collect telemetry per stage:
- intent detected
- candidate sources
- candidate count per source
- identity resolution scores
- filtering steps
- ranking components
- rejection reasons
Output structured JSON/text.

## Phase 2 – Candidate/Ranking Separation
Introduce internal structs: Candidate, CandidateSet, CandidateSource, RetrievalSignals, RankingPolicy.
Keep QueryEngine API compatible. New internal pipeline.

## Phase 3 – Intent Classification
Deterministic classifier based on query patterns:
"What calls X?" → CallerLookup
"What does X call?" → CalleeLookup
etc.
Intent influences candidate sources, relationship types, expansion depth.

## Phase 4 – Entity Resolution
Ranked identity resolution using qualified name, namespace, module, entity type, imports.

## Phase 5 – Intent-Aware Expansion
Per-intent expansion policies with max depth/nodes/edges.

## Phase 6 – Deterministic Ranking
Transparent scoring with configurable weights.

## Phase 7 – Strategy Matrix
A Baseline, B Identity First, C Intent First, D Two Stage, E Combined.

## Phase 8 – Metrics
Precision@k, Recall@k, MRR, nDCG, candidate/result ratio, evidence density.

## Deliverables
research/retrieval/* files updated, tracking.md updated with results.
