---
title: Structured Results
---

# Structured Results

Prime returns structured, typed results optimized for agent reasoning — not prose.

## Envelope

Every tool returns `PrimeEnvelope<T>`:

```json
{
  "status": "complete",
  "coverage": "full",
  "source_required": false,
  "provenance": {
    "kind": "exact",
    "source": "static symbol resolution",
    "confidence": "exact"
  },
  "warnings": [],
  "result": { ... }
}
```

## Result Types

### Search Result
```json
{
  "entities": [
    { "qualified_name": "AuthService.login", "kind": "Function", "score": 0.95 },
    { "qualified_name": "AuthService.logout", "kind": "Function", "score": 0.87 }
  ]
}
```

### Lookup Result
```json
{
  "entity": {
    "qualified_name": "AuthService.login",
    "kind": "Function",
    "signature": "fn(email: string, password: string) -> Result<Session, AuthError>",
    "documentation": "Authenticate user and create session",
    "file": "src/auth/login.ts",
    "line": 12,
    "confidence": "exact",
    "provenance": { "kind": "declared", "source": "static analysis" }
  }
}
```

### Context Result
```json
{
  "entity": "AuthService.login",
  "calls": [
    "UserRepository.findByEmail",
    "PasswordVerifier.verify",
    "SessionStore.create"
  ],
  "called_by": [
    "CheckoutController.authenticate",
    "AdminController.login"
  ],
  "dependencies": {
    "declared": ["UserRepository", "PasswordVerifier", "SessionStore"],
    "discovered": ["Logger"]
  }
}
```

### Impact Result
```json
{
  "entity": "AuthService.login",
  "direct_impact": ["CheckoutController.authenticate", "AdminController.login"],
  "transitive_impact": ["OrderService.placeOrder", "AdminDashboard.stats"],
  "tests_affected": [
    "AuthService.test.login_success",
    "AuthService.test.login_failure"
  ],
  "risk_score": 0.72
}
```

## Design Principles

| Principle | Implementation |
|-----------|----------------|
| No prose | Results are data, not text |
| Typed | Every field has a schema |
| Provenance | Every fact tagged with confidence |
| Source escalation | `source_required` flag |
| Token efficient | Minimal representation |
| Composable | Agents combine results |

## Next

- [Retrieval](./retrieval.md)
- [Source Escalation](./source-escalation.md)
- [Confidence](../specification/confidence.md)