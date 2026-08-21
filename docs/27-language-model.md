---
title: Language Model
---

# Language Model

Prime's universal semantic model — the common representation across all languages.

## Design Principle

```
language-specific analysis
          │
          ▼
  universal semantics
          │
          ▼
        PRIME
```

Language-specific frontends understand different concepts with different precision. Prime makes those differences explicit.

## Universal Types

| Universal Type | Maps To |
|----------------|---------|
| Function | fn, func, def, function, method |
| Class | class, struct, interface, trait, type |
| Module | module, package, namespace, crate |
| Variable | let, const, var, field, property |
| Type | type, interface, trait, class, struct |
| Call | call, invoke, apply |

## Language Capability Model

Each language frontend declares what it can extract:

```typescript
interface LanguageCapabilities {
  symbols: boolean;           // Symbol extraction
  calls: boolean;             // Call graph
  imports: boolean;           // Import graph
  types: boolean;             // Type signatures
  generics: boolean;          // Generic type params
  macros: boolean;            // Macro expansion
  reflection: boolean;        // Runtime reflection hints
  generics: boolean;          // Generated code
  confidence: ConfidenceMap;  // Per-feature confidence
}
```

### Current Capabilities

| Language | Symbols | Calls | Imports | Types | Generics | Macros | Reflection |
|----------|---------|-------|---------|-------|----------|--------|------------|
| TypeScript | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ⚠️ |
| JavaScript | ✅ | ✅ | ✅ | ⚠️ | ❌ | ❌ | ⚠️ |
| Python | ✅ | ✅ | ✅ | ⚠️ | ❌ | ❌ | ✅ |
| Rust | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ⚠️ |
| Go | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ⚠️ |
| Java | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ |
| C | ✅ | ✅ | ✅ | ⚠️ | ❌ | ✅ | ❌ |
| C++ | ✅ | ✅ | ✅ | ⚠️ | ✅ | ✅ | ❌ |

## Cross-Language Relationships

Prime tracks relationships across language boundaries:

```typescript
// TypeScript
fetch('/api/users')  // HTTP call
```

```rust
// Rust
#[get("/users")]
fn get_users() -> Json<Vec<User>>  // HTTP endpoint
```

Prime derives: `TypeScript:fetch → HTTP → Rust:get_users`

### Relationship Types

| Type | Example |
|------|---------|
| HTTP | REST, GraphQL, gRPC |
| FFI | C ↔ Rust, JNI, WASM |
| IPC | Message queues, pipes |
| Database | Shared schema |
| Config | Shared YAML/JSON/TOML |

## Confidence Across Languages

| Scenario | Confidence |
|----------|------------|
| Same-language static call | exact |
| Cross-language HTTP (OpenAPI) | derived |
| Cross-language FFI (bindgen) | exact |
| Cross-language HTTP (no spec) | inferred |
| Dynamic dispatch (any) | inferred |
| Reflection (any) | inferred |

## Language-Agnostic Queries

Queries work uniformly across languages:

```json
{
  "intent": "context",
  "target": "UserService.getUser",
  "depth": 2
}
```

Returns unified context regardless of implementation language.

## Open Questions

- How to represent language-specific concepts (traits, interfaces, typeclasses) uniformly?
- What is the confidence degradation across language boundaries?
- How to handle language-specific metaprogramming (macros, templates, reflection)?
- Should Prime have a "language capability model" describing what each frontend can extract?

## Next

- [Provenance](./provenance.md)
- [Confidence](./confidence.md)
- [Performance](./performance.md)