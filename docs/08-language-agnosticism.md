---
title: Language Agnosticism
---

# Language Agnosticism

## The Requirement

Prime must work across programming languages. This is a hard requirement.

It should be able to process:

```
TypeScript
JavaScript
Python
Rust
Go
Java
Kotlin
C
C++
C#
Ruby
PHP
Swift
Scala
Dart
Lua
and others
```

It must also handle **polyglot repositories**:

```
frontend/
    TypeScript

backend/
    Rust

services/
    Go

native/
    C++

automation/
    Python

infrastructure/
    Terraform
    YAML
```

The representation must preserve meaningful relationships across those boundaries.

For example:

```
TypeScript
    │
    ▼
HTTP API
    │
    ▼
Rust service
    │
    ▼
gRPC
    │
    ▼
Go service
    │
    ▼
database
```

## Universal Model Design

The universal model should not attempt to make every language identical.

Instead:

```
language-specific analysis
          │
          ▼
  universal semantics
          │
          ▼
        PRIME
```

Language-specific frontends may understand different concepts with different levels of precision.

Prime should make those differences explicit.

## Current Implementation Status

| Language | Parser | Symbols | Relations | Types | Status |
|----------|--------|---------|-----------|-------|--------|
| TypeScript | ✅ | ✅ | ✅ | ✅ | Production |
| JavaScript | ✅ | ✅ | ✅ | ⚠️ | Production |
| Python | ✅ | ✅ | ✅ | ⚠️ | Production |
| Rust | ✅ | ✅ | ✅ | ✅ | Production |
| Go | ✅ | ✅ | ✅ | ✅ | Production |
| Java | ✅ | ✅ | ✅ | ✅ | Production |
| C | ✅ | ✅ | ⚠️ | ⚠️ | Beta |
| C++ | ✅ | ✅ | ⚠️ | ⚠️ | Beta |
| C# | ❌ | ❌ | ❌ | ❌ | Planned |
| Swift | ❌ | ❌ | ❌ | ❌ | Planned |
| Kotlin | ❌ | ❌ | ❌ | ❌ | Planned |

## Cross-Language Relationships

Prime must track relationships across language boundaries:

```typescript
// TypeScript
fetch('/api/users')  // HTTP call
```

```rust
// Rust
#[get("/users")]
fn get_users() -> Json<Vec<User>>  // HTTP endpoint
```

Prime should derive: `TypeScript:fetch → HTTP → Rust:get_users`

## Open Questions

- How to represent language-specific concepts (traits, interfaces, typeclasses) uniformly?
- What is the confidence degradation across language boundaries?
- How to handle language-specific metaprogramming (macros, templates, reflection)?
- Should Prime have a "language capability model" describing what each frontend can extract?

## Next

- [Compression](./compression.md)
- [Information Theory](./information-theory.md)
- [Specification: Language Model](../specification/language-model.md)