---
title: Codebase Knowledge
---

# Codebase Knowledge Research

## What Information Exists in a Codebase?

A codebase contains multiple layers of information:

| Layer | Examples | Extractability |
|-------|----------|----------------|
| Syntax | Tokens, AST, formatting | Trivial (Tree-sitter) |
| Symbols | Functions, classes, variables, types | High (language servers) |
| Relationships | Calls, imports, extends, implements | High (static analysis) |
| Types | Signatures, generics, inference | Medium (type checkers) |
| Control flow | Branches, loops, exceptions | Medium (CFG) |
| Data flow | Variable assignments, mutations | Hard (dataflow analysis) |
| Contracts | Pre/post conditions, invariants | Low (requires specs) |
| Behavior | Runtime patterns, side effects | Very hard (dynamic) |
| Intent | Design decisions, rationale | Human-only |

## What Agents Actually Need

Agents don't need all layers equally. Research suggests:

### High Utility (Prime Priority)
- **Symbol identity** — What is this thing?
- **Call graph** — Who calls whom?
- **Dependency graph** — What depends on what?
- **Type signatures** — What does this function take/return?
- **Module boundaries** — What belongs together?

### Medium Utility
- **Control flow** — For impact analysis
- **Data flow** — For security/reliability
- **Test relationships** — What tests what?

### Low Utility (Defer)
- **Full AST** — Too verbose
- **Formatting** — Irrelevant
- **Comments** — Often stale
- **Runtime behavior** — Requires execution

## The Universal Model Challenge

Different languages expose different information:

| Language | Static Types | Reflection | Macros | Generated Code |
|----------|--------------|------------|--------|----------------|
| Rust | ✅ | Limited | ✅ | ✅ (build.rs) |
| TypeScript | ✅ | ✅ | ❌ | ✅ |
| Python | Gradual | ✅ | ❌ | ❌ |
| Go | ✅ | Limited | ❌ | ✅ |
| Java | ✅ | ✅ | ❌ (annotations) | ✅ |
| C++ | ✅ | ❌ | ✅ (templates) | ❌ |

Prime's universal model must handle these differences explicitly, not paper over them.

## Confidence Propagation

Information extracted at different confidence levels:

```
exact (static resolution) → derived (transitive) → inferred (heuristic) → unknown
```

Each transformation must track confidence degradation.

## Next

- [Minimum Knowledge Unit](./minimum-knowledge-unit.md)
- [Language Agnosticism](./language-agnosticism.md)
- [Specification: Knowledge Model](../specification/knowledge-model.md)