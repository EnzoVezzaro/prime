# Explain Mode Telemetry Schema

## QueryEnvelope
{
  "query": string,
  "original_query": string,
  "intent": {"type": string, "confidence": float, "pattern": string},
  "parameters": {}
}

## CandidateGeneration
{
  "sources": [
    {"name": "fulltext", "candidates": int, "time_ms": float},
    {"name": "type_index", "candidates": int, "time_ms": float},
    {"name": "name_index", "candidates": int, "time_ms": float}
  ],
  "total_candidates": int,
  "deduplicated": int
}

## IdentityResolution
{
  "candidates_before": int,
  "candidates_after": int,
  "resolution": [
    {"entity_id": string, "score": float, "reasons": [string]}
  ]
}

## Filtering
{
  "steps": [
    {"name": string, "before": int, "after": int, "reason": string}
  ]
}

## Ranking
{
  "signals": [
    {"name": string, "weight": float, "description": string}
  ],
  "top_k": [
    {"entity_id": string, "score": float, "signal_breakdown": {}}
  ]
}

## Evidence
{
  "nodes": int,
  "edges": int,
  "bytes": int,
  "tokens": int
}

## Result
{
  "final_entities": int,
  "final_relationships": int,
  "evidence_density": float,
  "candidate_result_ratio": float,
  "latency_ms": float
}

## Output Format
JSON or human readable tree.
