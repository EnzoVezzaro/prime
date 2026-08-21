---
title: Minimum Knowledge Unit
---

# Minimum Knowledge Unit

## The Research Question

> **What is the smallest useful unit of knowledge an agent needs?**

This is one of Prime's most important research areas.

The goal is not:
> "How do we store every piece of code?"

The goal is:
> "What is the smallest useful unit of knowledge an agent needs?"

## Candidate Units

| Unit | Pros | Cons |
|------|------|------|
| Entity (symbol) | Fine-grained, composable | May lack context |
| Fact (relation) | Atomic, verifiable | Needs entity context |
| Contract | High utility, semantic | Language-specific |
| Behavioral fact | Captures runtime behavior | Hard to extract statically |
| Dependency | Structural, actionable | Loses semantic detail |
| State transition | Models behavior | Complex to represent |

## Illustrative Example

```
AuthService.login
    ── CALLS ──> UserRepository.findByEmail
    ── CALLS ──> PasswordVerifier.verify
    ── CREATES ──> Session
    ── DEPENDS_ON ──> SessionStore
    ── RETURNS ──> Session
    ── MAY_THROW ──> InvalidCredentials
    ── MAY_THROW ──> UserNotFound
```

With metadata:
```
confidence: exact
provenance: declared
source: src/auth/login.ts:12
```

Versus inferred:
```
PluginManager
    ── MAY_CALL ──> PaymentProvider

confidence: inferred
reason: dynamic dispatch
```

## Research Directions

### 1. Entity-Centric
Store entities with embedded relations. Query: "give me entity X with its neighborhood."

### 2. Fact-Centric
Store atomic facts (subject, predicate, object, metadata). Query: "give me all facts about X."

### 3. Contract-Centric
Store behavioral contracts (preconditions, postconditions, invariants). Query: "what does X guarantee?"

### 4. Hybrid
Multiple unit types with explicit relationships between them.

## Open Questions

- Is there a single universal unit, or a small set of primitives?
- How do units compose into higher-level understanding?
- What metadata is essential per unit (confidence, provenance, source, version)?
- How do units handle uncertainty (dynamic dispatch, reflection, macros)?
- How do units scale from tiny to enormous codebases?
- What is the token cost of each unit type for agent consumption?

## Next

- [Knowledge Model](../specification/knowledge-model.md)
- [Representation](../specification/representation.md)
- [Provenance](../specification/provenance.md)