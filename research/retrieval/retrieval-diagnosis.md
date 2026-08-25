# Retrieval Diagnosis – Current Prime Pipeline

**Date:** 2026-08-21
**Commit:** a0d50ff33044
**Status:** Diagnostic – no code changes

## Summary Hypothesis

Strong knowledge substrate + weak retrieval → low precision / noisy context.

Evidence: Overall accuracy 62.7%, entity precision 1-5% with recall 21-100% across repos. Relationship precision high, relationship recall moderate. Latency excellent.

## 1. Where candidates are generated

- `QueryEngine::search` – iterates over **all entities** in `graph.entities`, computes textual score.
- `QueryEngine::find_by_name` / `find_by_prefix` – uses `graph.find_by_name` / `find_by_prefix` name index, returns all IDs matching name/prefix.
- `ToolExecutor::prime_context` – resolves entity by qualified name then pulls `all_incoming` / `all_outgoing` relations.
- `ToolExecutor::prime_relationships` / `prime_dependencies` / `prime_impact` – pull direct graph neighbors via `graph.dependencies/dependents/callers/callees/all_incoming/all_outgoing`.

Candidate generation is **graph-wide or name-index-wide**, with no intent gating.

## 2. Where candidates are filtered

- Confidence filter: `entity.confidence >= opts.min_confidence`.
- `max_results` truncation after scoring.
- Relation-kind filter in `get_relations` via `opts.relation_kinds`.
- Quality filter in `ProgressiveContextBuilder` via token budget.

No semantic filtering by entity type, module, namespace, relationship relevance, or query intent.

## 3. Where candidates are ranked

- `QueryEngine::search` scoring:
  * exact name =100
  * name starts with =90
  * name contains =70
  * qualified name ends with `::query` =60
  * qualified name contains =40
  * doc contains =10
- Sorted by score then name.
- `get_context` score is hardcoded to 1.0.
- No ranking in `prime_context`, `prime_relationships`, `prime_dependencies`, `prime_impact` beyond truncation.

Ranking is **coupled to candidate generation** in `search` and is purely lexical.

## 4. Coupling of generation & ranking

Yes. `search` generates candidates by scanning all entities and ranks by lexical score in the same loop. No separate candidate pool, no recall-first stage, no re-ranking.

## 5. Entity identity resolution

- `find_by_qualified` – exact qualified name lookup via `name_index`.
- `find_by_name` – returns all entities with simple name match.
- No disambiguation, no qualified-name preference, no namespace/module weighting, no alias/import resolution.
- Ambiguous queries resolve to **all** matches, leading to over-generation.

## 6. How relationships affect retrieval

- Relationships are retrieved **after** entity resolution, not used for scoring.
- `prime_context` aggregates callers/callees/deps/dependents via `all_incoming/outgoing` without scoring or pruning.
- `prime_relationships` returns raw neighbor lists.
- No relationship-aware ranking, no graph distance weighting, no typed expansion policies.

## 7. Evidence selection

Evidence not explicitly modeled. `PrimeEnvelope` carries `source_required`, `coverage`, `missing`, `warnings`, `provenance`. Provenance is empty in current tools. No evidence density metric, no distinction between direct evidence vs inferred neighbor.

## 8. Irrelevant knowledge entry points

- Lexical search over entire corpus → many false positives.
- `max_results` default 50 for agent, no relevance threshold.
- Context building pulls **all** direct neighbors regardless of intent.
- No max depth enforcement in tools aside from `QueryOptions.max_depth` which is not applied in `get_surrounding_context`.
- No penalty for distance, test code, or unrelated entity types.

## 9. Query intent understanding

**No.** `ToolIntent` enum exists `Search, Lookup, Context, Relationships, Dependencies, Impact, Architecture`. No classification of natural language query into SymbolLookup/Definition/Callers/etc. Intent does not change candidate sources, expansion depth, or ranking signals.

## 10. Determinism

Retrieval is deterministic: same query → same order. However, ranking is weak and over-generative, making determinism insufficient for precision.

## Precision Bottlenecks Identified

1. **Candidate over-generation**: search scans all entities with lexical match; no type/module/namespace constraints.
2. **Lexical ranking only**: no qualified name, identity, intent, relationship, or evidence signals.
3. **No intent-aware expansion**: context always pulls callers+callees+deps+dependents → noise.
4. **Entity resolution ambiguity**: simple name → many entities, no disambiguation.
5. **Relationship noise**: all incoming/outgoing edges returned, no filtering by kind relevance.
6. **No evidence filtering**: no distinction between definition vs reference vs inferred.
7. **Missing ranking transparency**: no explainability for why an entity was selected.

## Immediate Diagnostic Needs

- `--explain` mode exposing pipeline stages and per-candidate scores.
- Per-query candidate count → filtered count → result count.
- Intent classification prototype.
- Separate candidate generation vs ranking experiments.

## Confidence

FACT: code inspection of `prime-index/src/query.rs` and `prime-index/src/tools.rs`.
OBSERVATION: benchmark precision/recall patterns.
HYPOTHESIS: weak retrieval causes low precision – needs experimental validation.
