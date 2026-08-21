---
title: Source Escalation
---

# Source Escalation

Prime minimizes source retrieval. It does not eliminate it at all costs.

## The Principle

```
Agent question
      ↓
Prime
      ↓
Can Prime answer?
YES
  ↓
Answer (source_required: false)

NO / PARTIAL
  ↓
Targeted source retrieval (source_required: true)
```

## Why Not Eliminate Source?

1. **Correctness** — Some questions need source (dynamic behavior, complex logic)
2. **Trust** — Agents must verify Prime's answers
3. **Completeness** — Prime is lossy by design
4. **Evolution** — Source is ground truth; Prime is derived

## Source Required Flag

Every `PrimeEnvelope` includes `source_required: boolean`:

- `false` — Prime's answer is complete and trusted
- `true` — Agent should fetch source for verification or completion

## Escalation Triggers

| Trigger | Example | Escalation |
|---------|---------|------------|
| Dynamic dispatch | `interface.method()` | Fetch implementations |
| Reflection | `Class.forName()` | Fetch registration |
| Macros | `macro_rules!` | Fetch expansion |
| Generated code | `build.rs` output | Fetch generator |
| Low confidence | `confidence: inferred` | Verify in source |
| Missing provenance | No analysis available | Full source read |

## Targeted Retrieval

When escalating, Prime provides hints:

```json
{
  "source_required": true,
  "escalation_hints": {
    "files": ["src/payments/stripe.rs", "src/payments/paypal.rs"],
    "symbols": ["PaymentProvider", "process_payment"],
    "reason": "dynamic dispatch on trait PaymentProvider"
  }
}
```

Agent fetches only what's needed.

## Confidence and Escalation

| Confidence | Escalation Likelihood |
|------------|----------------------|
| exact | Never |
| derived | Rare (verify transitive) |
| inferred | Often (verify heuristic) |
| unknown | Always |

## Agent Workflow

```
1. Ask Prime
2. If source_required: false → use answer
3. If source_required: true →
   a. Read escalation_hints
   b. Fetch minimal source
   c. Combine with Prime's answer
   d. Cache for future
```

## Next

- [Retrieval](./retrieval.md)
- [Confidence](../specification/confidence.md)
- [Provenance](../specification/provenance.md)