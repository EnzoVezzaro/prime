# Evidence Filtering & Bounded Graph Expansion

## Problem
Context expansion returns unbounded neighborhoods causing low evidence density.

## Goals
- Bounded expansion per intent
- Evidence density = relevant / total
- Progressive disclosure

## Expansion Policies
- Lookup: depth 1, max 10 nodes
- Search: depth 1, max 20 nodes
- Callers/Callees: depth 2, max 30 nodes, filter by relation type
- Context: depth 2, max 50 nodes

## Filtering Steps
1. Confidence filter
2. Relation type filter
3. Entity kind filter
4. Token budget filter
5. Duplicate removal

## Progressive Disclosure
Return first layer, allow expand request.

## Implementation
Add get_context_with_telemetry returning telemetry with nodes/edges/bytes.

## Metrics
evidence_density, bytes_retrieved, tokens_retrieved.
