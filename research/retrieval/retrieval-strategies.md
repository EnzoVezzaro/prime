# Retrieval Strategies Experiment

## A. Baseline
Current Prime retrieval. Control.

## B. Exact semantic identity
Prefer exact name, qualified name, path, module, namespace. Score identity > textual.

## C. Intent-aware
Classify query intent, restrict candidate types and relationships.

## D. Two-stage
Stage1 high-recall candidate generation. Stage2 deterministic ranking.

## E. Bounded semantic graph
Expand from resolved entity via intent-relevant relationships only.

Metrics: P@1, P@3, P@5, P@10, Recall@10, MRR, nDCG, evidence density, candidate→result ratio.

Do not combine until individual evaluated.
