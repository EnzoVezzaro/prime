---
title: Agent Tools
---

# Agent Tools

Prime exposes 7 semantic tools via MCP.

## Tool Reference

### `prime_search`

Search entities by keyword.

**Request:**
```json
{
  "intent": "search",
  "target": "AuthService",
  "limit": 50,
  "confidence": "high"
}
```

**Response:**
```json
{
  "status": "complete",
  "result": {
    "entities": [
      { "qualified_name": "AuthService.login", "kind": "Function", "score": 0.95 },
      { "qualified_name": "AuthService.logout", "kind": "Function", "score": 0.87 }
    ]
  }
}
```

### `prime_lookup`

Exact entity by qualified name.

**Request:**
```json
{
  "intent": "lookup",
  "target": "AuthService.login"
}
```

**Response:**
```json
{
  "status": "complete",
  "result": {
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
}
```

### `prime_context`

Knowledge neighborhood (dependencies, callers, callees).

**Request:**
```json
{
  "intent": "context",
  "target": "AuthService.login",
  "depth": 2,
  "token_budget": 16384
}
```

**Response:**
```json
{
  "status": "complete",
  "source_required": false,
  "result": {
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
}
```

### `prime_relationships`

Relationships across dimensions.

**Request:**
```json
{
  "intent": "relationships",
  "target": "User",
  "scope": "all"
}
```

**Response:**
```json
{
  "status": "complete",
  "result": {
    "entity": "User",
    "relations": [
      { "kind": "DEFINES", "target": "User.id" },
      { "kind": "DEFINES", "target": "User.email" },
      { "kind": "IMPLEMENTS", "target": "Identifiable" },
      { "kind": "USES", "target": "EmailValidator" },
      { "kind": "CALLED_BY", "target": "AuthService.login" }
    ]
  }
}
```

### `prime_dependencies`

Dependency graph.

**Request:**
```json
{
  "intent": "dependencies",
  "target": "AuthService",
  "transitive": true
}
```

**Response:**
```json
{
  "status": "complete",
  "result": {
    "entity": "AuthService",
    "dependencies": [
      { "target": "UserRepository", "kind": "declared", "confidence": "exact" },
      { "target": "PasswordVerifier", "kind": "declared", "confidence": "exact" },
      { "target": "SessionStore", "kind": "declared", "confidence": "exact" },
      { "target": "Logger", "kind": "discovered", "confidence": "exact" }
    ],
    "dependents": [
      "CheckoutController",
      "AdminController",
      "SessionRefreshJob"
    ]
  }
}
```

### `prime_impact`

Change impact analysis.

**Request:**
```json
{
  "intent": "impact",
  "target": "AuthService.login",
  "radius": 2
}
```

**Response:**
```json
{
  "status": "complete",
  "result": {
    "entity": "AuthService.login",
    "direct_impact": [
      "CheckoutController.authenticate",
      "AdminController.login"
    ],
    "transitive_impact": [
      "OrderService.placeOrder",
      "AdminDashboard.stats"
    ],
    "tests_affected": [
      "AuthService.test.login_success",
      "AuthService.test.login_failure",
      "CheckoutController.test.authenticate"
    ],
    "risk_score": 0.72
  }
}
```

### `prime_architecture`

System architecture overview.

**Request:**
```json
{
  "intent": "architecture",
  "target": ""
}
```

**Response:**
```json
{
  "status": "complete",
  "result": {
    "modules": [
      { "name": "auth", "entities": 12, "deps": ["database", "crypto"] },
      { "name": "checkout", "entities": 8, "deps": ["auth", "payments"] },
      { "name": "payments", "entities": 15, "deps": ["database", "external"] }
    ],
    "boundaries": [
      { "from": "checkout", "to": "ui", "violation": true }
    ],
    "layers": [
      { "name": "domain", "modules": ["auth", "payments"] },
      { "name": "application", "modules": ["checkout"] }
    ]
  }
}
```

## Common Parameters

| Parameter | Tools | Description |
|-----------|-------|-------------|
| `target` | All | Qualified name or search query |
| `limit` | search | Max results (default: 50) |
| `depth` | context | Traversal depth (default: 2) |
| `token_budget` | context | Max tokens in response (default: 16384) |
| `confidence` | search, lookup | Minimum confidence (exact/derived/inferred) |
| `transitive` | dependencies | Include transitive deps (default: false) |
| `scope` | relationships | Filter: all/calls/dependencies/implements |
| `radius` | impact | Transitive impact radius (default: 2) |

## Next

- [MCP](./mcp.md)
- [Structured Results](./structured-results.md)
- [Retrieval](./retrieval.md)