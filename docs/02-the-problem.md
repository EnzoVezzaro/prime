---
title: The Problem
---

# The Problem

## Agents Keep Rediscovering the Codebase

Current agents operate in a loop:

```
search
    ↓
read
    ↓
follow references
    ↓
inspect files
    ↓
infer relationships
    ↓
repeat
```

For a question like "Who calls `AuthService.login`?", the agent should not need to:

```
read files
→ search symbols
→ follow imports
→ parse declarations
→ inspect references
→ reconstruct relationships
```

## The Deeper Problem

The problem is not simply context-window size.

The deeper problem is **repeated reconstruction of knowledge that could have been derived once**.

Every agent session. Every question. Every repository. The same reconstruction.

## What Agents Actually Need

For "Who calls `AuthService.login`?":
```
AuthService.login
    ← CheckoutController
    ← AdminController
    ← SessionRefreshJob
```

For "What does `AuthService.login` depend on?":
```
AuthService.login
    → UserRepository.findByEmail
    → PasswordVerifier.verify
    → SessionStore.create
```

For "What can `AuthService.login` return or throw?":
```
returns:
    Session

may throw:
    UserNotFound
    InvalidCredentials
```

The source implementation is **not required** for these answers.

## Why Current Approaches Fall Short

| Approach | Limitation |
|----------|------------|
| Raw file access | Too slow, too much noise |
| Repository maps (Aider) | Good for LLM context, not structured retrieval |
| LSIF/SCIP | Language-server focused, not agent-optimized |
| Graph databases | Low-level primitives, not semantic tools |
| Vector search | No provenance, no exact answers, hallucinations |
| Embeddings + RAG | Lossy, no guarantees, source retrieval still needed |

## The Missing Layer

There is no **derived knowledge layer** optimized for:
- **Semantic queries** (not keyword search)
- **Provenance tracking** (declared vs discovered vs inferred)
- **Confidence levels** (exact, derived, inferred, unknown)
- **Agent tool schemas** (structured results, not prose)
- **Source escalation** (targeted retrieval when needed)

Prime is that layer.

## Next

- [What is Prime?](../01-what-is-prime.md)
- [Principles](../03-principles.md)
- [Research: Agent Architecture](../research/agent-architecture.md)