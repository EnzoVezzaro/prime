# Intent Classification Design

## Goal
Deterministic intent classification to drive candidate generation, filtering, ranking weights, and expansion policies.

## Intents
- Lookup: exact qualified name
- Prefix: starts with
- Search: free text
- Callers: who calls X
- Callees: who X calls
- Dependents: what depends on X
- Dependencies: what X depends on
- Context: neighborhood

## Classification Rules
Pattern based, no ML.

Examples:
- query ends with "calls" or starts with "who calls" → Callers
- query contains "()" → likely function
- query contains "::" → qualified name → Lookup
- query is single token → Search

## Implementation
Function classify_intent(query: &str, qtype: &str) -> Intent

Intent enum with variants and associated ranking policy.

## Telemetry
Include intent in RetrievalTelemetry.
