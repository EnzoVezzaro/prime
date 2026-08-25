# Candidate/Ranking Separation Design

## Problem
Current search mixes candidate generation and ranking in a single loop, making signals opaque and untestable.

## Goal
Separate pipeline into:
1. Candidate Generation
2. Candidate Enrichment
3. Filtering
4. Ranking
5. Result Materialization

## Components

### Candidate
{
  entity_id,
  source: Fulltext|TypeIndex|NameIndex,
  raw_score,
  signals: {}
}

### CandidateSet
Vector<Candidate> with dedup by entity_id.

### CandidateGeneration
Independent generators:
- FulltextGenerator: name/qualified_name/doc contains query
- NameIndexGenerator: prefix/exact on name_index
- TypeIndexGenerator: filter by kind

Each returns candidates with raw_score.

### Enrichment
Add signals per candidate:
- name_exact_match
- name_prefix_match
- name_contains
- qualified_ends_with
- qualified_contains
- doc_contains
- entity_type_match
- confidence

### Filtering
Deterministic filters:
- min_confidence
- entity_type_allowed
- language_allowed

### Ranking
Linear score = Σ weight_i * signal_i
Weights configurable per intent.

### API Changes
QueryEngine.search → returns Vec<EntitySummary>
New internal: generate_candidates → CandidateSet
New internal: rank_candidates → Vec<Candidate>

Keep public API compatible.

## Deliverables
- Candidate, CandidateSet structs
- CandidateGenerator trait
- Enrichment step
- RankingPolicy struct
- Telemetry updated with per-stage counts
