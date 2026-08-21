---
title: Information Theory
---

# Information Theory Research

## Core Concepts Applied to Codebase Knowledge

### Entropy
Measure of uncertainty in codebase information. High entropy = more information needed to describe.

```
H(X) = -Σ p(x) log₂ p(x)
```

For codebases: entropy of symbol names, relationship types, file structures.

### Information Bottleneck
Find the minimal representation that preserves relevant information for a task.

```
min I(X; T) - β I(T; Y)
```

Where:
- X = source code
- T = Prime representation (bottleneck)
- Y = agent questions
- β = tradeoff parameter

### Rate-Distortion Theory
How much can we compress before losing utility for agent questions?

```
R(D) = min I(X; X̂)  subject to E[d(X, X̂)] ≤ D
```

Where distortion d measures "agent question answerability loss."

### Minimum Description Length (MDL)
Best model = shortest description of data + model.

```
L(D, M) = L(M) + L(D|M)
```

For Prime: find representation minimizing description length while preserving agent utility.

## Application to Prime

### What is the "Relevant Information"?
Not all code information is relevant to agents. Research must identify:

1. **Question distribution** — What do agents actually ask?
2. **Information content** — How much info per entity/relation?
3. **Redundancy** — What's derivable from other facts?
4. **Noise** — What's formatting, boilerplate, comments?

### Measuring Utility

| Metric | Definition |
|--------|------------|
| Source-free answer rate | % questions answered without source |
| Tokens per answer | Total tokens in tool results / questions |
| Precision@K | Correct entities in top-K results |
| Recall@K | True positives found in top-K |

### Theoretical Limits

- **Lower bound**: Entropy of agent question distribution
- **Upper bound**: Full source code size
- **Prime target**: 1-2% of source size with >80% answer rate

## Open Questions

- What is the actual entropy of agent question distributions?
- Can we learn the bottleneck from agent interaction logs?
- How does distortion correlate with agent task failure?
- Is there a phase transition in utility vs compression ratio?

## References

- Tishby, Pereira, Bialek (1999) — Information Bottleneck
- Shannon (1948) — Mathematical Theory of Communication
- Rissanen (1978) — Minimum Description Length
- Cover & Thomas — Elements of Information Theory

## Next

- [Indexing](./indexing.md)
- [Storage](./storage.md)
- [Specification: Representation](../specification/representation.md)