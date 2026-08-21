---
title: Retrieval
---

# Retrieval

Prime's retrieval is optimized for agent queries — fast, precise, token-efficient.

## Retrieval Strategies

| Strategy | Use Case | Latency |
|----------|----------|---------|
| Exact lookup | `prime_lookup` | ~50µs |
| Prefix search | `prime_search` (prefix) | ~100µs |
| Keyword search | `prime_search` (keyword) | ~200µs |
| Graph traversal | `prime_context`, `prime_dependencies` | ~500µs |
| Impact analysis | `prime_impact` | ~1ms |
| Architecture | `prime_architecture` | ~2ms |

## Indexes

| Index | Purpose |
|-------|---------|
| Name index | Exact qualified name → entity ID |
| Prefix index | Prefix → entity IDs (for autocomplete) |
| Keyword index | Token → entity IDs (for search) |
| Relation index | Outgoing/incoming edges per entity |
| Dependency bitmaps | Roaring bitmaps for transitive closure |

## Progressive Context

`prime_context` supports progressive disclosure:

```json
{
  "target": "AuthService.login",
  "depth": 1,
  "token_budget": 4096
}
```

Returns minimal context. Agent can request deeper:
```json
{
  "target": "AuthService.login",
  "depth": 3,
  "token_budget": 32768
}
```

## Token Budget

Results respect `token_budget`:
- Truncates least-relevant relations first
- Preserves declared over discovered
- Preserves exact over inferred
- Returns `coverage: "partial"` if truncated

## Source Escalation

When Prime cannot answer:
```json
{
  "status": "partial",
  "source_required": true,
  "provenance": { "kind": "inferred", "source": "dynamic dispatch" },
  "warnings": ["Dynamic dispatch: exact targets unknown"],
  "result": { "possible_targets": ["PaymentProvider.stripe", "PaymentProvider.paypal"] }
}
```

Agent sees `source_required: true` and fetches source.

## Caching

- Query results cached in-memory (LRU, 10k entries)
- Cache keyed by (tool, arguments, artifact_version)
- Invalidate on `prime build`
- TTL: 5 minutes default

## Next

- [Source Escalation](./source-escalation.md)
- [Indexing](../research/indexing.md)
- [Storage](../research/storage.md)