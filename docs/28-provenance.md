---
title: Provenance
---

# Provenance

Prime tracks the origin of every fact — written by a human, observed in code, or guessed by a tool.

## Provenance Types

```typescript
type Provenance =
  | { kind: 'declared'; source: 'AGENTS.md' }
  | { kind: 'discovered'; source: 'static_analysis' | 'imports' | 'calls' }
  | { kind: 'inferred'; source: 'heuristic' | 'dynamic_dispatch' | 'pattern'; reason: string }
  | { kind: 'memory'; source: '.acc-memory.md' }
  | { kind: 'stored'; source: 'prime_artifact' }
```

## Provenance Hierarchy

When facts conflict, provenance determines priority:

```
declared (human) > discovered (static) > inferred (heuristic) > memory > stored
```

## Provenance in Practice

### Declared (Human)
```json
{
  "entity": "AuthService.login",
  "relation": "DEPENDS_ON",
  "target": "UserRepository",
  "provenance": { "kind": "declared", "source": "AGENTS.md" }
}
```
- Written in `AGENTS.md` by human
- Highest authority
- Overrides discovered/inferred

### Discovered (Static Analysis)
```json
{
  "entity": "AuthService.login",
  "relation": "CALLS",
  "target": "UserRepository.findByEmail",
  "provenance": { "kind": "discovered", "source": "static_analysis" }
}
```
- Observed in source code
- Verifiable by re-running analysis
- High confidence

### Inferred (Heuristic)
```json
{
  "entity": "PluginManager",
  "relation": "MAY_CALL",
  "target": "PaymentProvider",
  "provenance": { "kind": "inferred", "source": "dynamic_dispatch", "reason": "trait object call" }
}
```
- Probabilistic/heuristic
- Never treated as fact without verification
- Triggers source escalation

### Memory (Agent Knowledge)
```json
{
  "entity": "AuthService.login",
  "relation": "GOTCHA",
  "target": "non-reentrant decode()",
  "provenance": { "kind": "memory", "source": ".acc-memory.md" }
}
```
- From `.acc-memory.md` (gitignored)
- Durable agent knowledge
- Lower than declared/discovered

### Stored (Artifact)
```json
{
  "entity": "AuthService.login",
  "relation": "CALLS",
  "target": "UserRepository.findByEmail",
  "provenance": { "kind": "stored", "source": "prime_artifact" }
}
```
- Loaded from Prime artifact
- Chain traces back to original provenance
- Used for incremental updates

## Provenance in Envelope

Every `PrimeEnvelope` includes provenance:

```json
{
  "status": "complete",
  "provenance": {
    "kind": "exact",
    "source": "static symbol resolution",
    "confidence": "exact"
  },
  "result": { ... }
}
```

## Agent Behavior by Provenance

| Provenance | Agent Should |
|------------|--------------|
| `declared` | Trust without verification |
| `discovered` | Trust, optionally verify |
| `inferred` | Treat as hypothesis, verify in source |
| `memory` | Trust as durable knowledge |
| `stored` | Trust, traceable to origin |

## Conflict Resolution

When multiple provenances exist for same fact:

1. **Exact match** → Merge, highest provenance wins
2. **Conflict** → Highest provenance wins, lower flagged
3. **Declared vs Discovered** → Declared wins (human intent)
4. **Inferred vs anything** → Inferred flagged, never overrides

## Next

- [Confidence](./confidence.md)
- [Performance](./performance.md)
- [Knowledge Model](../specification/knowledge-model.md)