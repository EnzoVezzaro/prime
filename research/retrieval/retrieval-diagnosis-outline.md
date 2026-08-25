# Retrieval Diagnosis Outline

**Status:** Planning
**Objective:** Determine why entity precision is extremely low while entity recall is moderate/high.

## 1. Benchmark Data Review
- Commit: a0d50ff33044
- Overall BMF knowledge accuracy: ~61%
- Source-free accuracy: ~59%
- Per-repo accuracy: Express 83.3%, Gin 73.9%, HTTPX 65.2%, Bat 56.0%, Spdlog 26.1%
- Entity precision/recall patterns:
  - httpx: precision 1.17% recall 60%
  - express: precision 4.81% recall 100%
  - gin: precision 1.83% recall 70.8%
  - bat: precision 1.18% recall 50%
  - spdlog: precision 5.63% recall 21%

## 2. Diagnostic Questions
1. What exactly counts as an entity match?
2. What entities are returned for each query?
3. How many candidates are generated?
4. How are candidates ranked?
5. Does retrieval return entities independently or connected subgraphs?
6. Are relationships influencing ranking?
7. Are files/modules influencing ranking?
8. Are language namespaces influencing ranking?
9. Are qualified names being used?
10. Are aliases/imports being resolved?
11. Is benchmark measuring retrieval precision against oversized result set?
12. Is evaluator contributing to extremely low precision?

## 3. Retrieval Pipeline Inspection
Stages to inspect:
- QUERY CLASSIFICATION
- ENTITY CANDIDATES
- LEXICAL MATCH
- SYMBOL MATCH
- PATH/MODULE MATCH
- RELATIONSHIP EXPANSION
- GRAPH RANKING
- FINAL RESULTS

Per candidate diagnostic fields:
entity, qualified_name, file, language, entity_type, match_score, name_score, path_score, namespace_score, relationship_score, semantic_score, confidence, distance

## 4. Evidence to Collect
- Query logs with candidate counts
- Ranking scores distribution
- Result set sizes per query
- Intent classification accuracy
- Entity identity collisions

## 5. Hypotheses
- H1: Candidate over-generation, no effective ranking
- H2: Qualified name/namespace not used in scoring
- H3: Evaluator counts all returned entities vs ground truth without threshold
- H4: C++ extraction issues dominate spdlog

## 6. Next Steps
- Implement `--explain` mode
- Run diagnostic queries on 5 repos
- Produce per-repo precision/recall breakdown
- Compare candidate generation strategies
