---
title: Confidence
---

# Confidence

Prime distinguishes four confidence levels for every fact.

## Levels

| Level | Meaning | Typical Source |
|-------|---------|----------------|
| `exact` | Verified in source | Static symbol resolution, declared in AGENTS.md |
| `derived` | Computed from exact facts | Transitive closure, type inference, control flow |
| `inferred` | Probabilistic/heuristic | Dynamic dispatch, reflection, macros, patterns |
| `unknown` | No evidence | Unanalyzed code, external dependencies, generated code |

## Rules

1. **Exact wins** — When exact and derived conflict, exact wins
2. **Derived from exact** — Derived facts trace to exact ancestors
3. **Inferred marked** — Inferred facts never silently become derived
4. **Unknown propagates** — Unknown in → unknown out

## Examples

### Exact
```json
{
  "entity": "AuthService.login",
  "relation": "CALLS",
  "target": "UserRepository.findByEmail",
  "confidence": "exact",
  "provenance": { "kind": "discovered", "source": "static_analysis" }
}
```

### Derived
```json
{
  "entity": "AuthService.login",
  "relation": "TRANSITIVE_DEPENDS_ON",
  "target": "DatabaseConnection",
  "confidence": "derived",
  "provenance": { "kind": "derived", "source": "transitive_closure", "path": ["AuthService.login", "UserRepository.findByEmail", "DatabaseConnection"] }
}
```

### Inferred
```json
{
  "entity": "PluginManager",
  "relation": "MAY_CALL",
  "target": "PaymentProvider",
  "confidence": "inferred",
  "provenance": { "kind": "inferred", "source": "dynamic_dispatch", "reason": "trait object call" }
}
```

### Unknown
```json
{
  "entity": "ExternalSDK",
  "relation": "CALLS",
  "target": "unknown",
  "confidence": "unknown",
  "provenance": { "kind": "unknown", "source": "external_dependency" }
}
```

## Agent Behavior

| Confidence | Agent Should |
|------------|--------------|
| `exact` | Trust without verification |
| `derived` | Trust, optionally verify transitive path |
| `inferred` | Treat as hypothesis, verify in source |
| `unknown` | Must fetch source |

## Next

- [Provenance](./provenance.md)
- [Knowledge Model](../specification/knowledge-model.md)