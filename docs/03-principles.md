---
title: Principles
---

# Principles

Prime follows evidence-based research principles. Every major conclusion distinguishes between:

- **FACT** — Verified by primary source, reproducible
- **OBSERVATION** — Directly observed, reproducible
- **HYPOTHESIS** — Proposed explanation, requires validation
- **INFERENCE** — Deduced from evidence, marked as such
- **OPEN QUESTION** — Explicitly unknown, needs research

## Core Principles

### 1. Evidence Over Assumptions
Never claim "X is faster" without evidence. Write: "Benchmark/source X reports...", "Our experiment indicates...", "Theoretical analysis suggests...", "This remains unverified."

### 2. Primary Sources First
Prefer official documentation, academic papers, technical specifications, original repositories, engineering papers, benchmark results, source code.

### 3. No Premature Convergence
Do NOT decide on technologies (SQLite, RocksDB, graph DB, Tree-sitter, protobuf, etc.) until research establishes why.

### 4. Research Alternatives Fairly
For every major design area, investigate multiple approaches.

### 5. No Product Design
Document evidence, alternatives, tradeoffs, hypotheses, open questions. Do NOT decide Prime's final architecture.

### 6. Source Remains Authoritative
Prime is a derived layer. The source repository is the ground truth. Prime never replaces source.

### 7. Agent-Native First
Every design decision measured against: does this help an agent answer questions with less source retrieval?

### 8. Language Agnostic by Design
Prime must work across programming languages. Not "support many parsers" — derive a universal semantic model.

### 9. Deliberately Lossy
Prime should be intentionally non-reversible. Discard what agents don't need.

### 10. Provenance and Confidence Are First-Class
Every fact tagged: exact, derived, inferred, unknown. Every fact traced to source.

### 11. Source Escalation, Not Elimination
Minimize source retrieval. Do not eliminate it at all costs. Prime admits uncertainty.

### 12. Research Determines Representation
The physical format (binary, graph, columnar, etc.) is decided by research, not preference.

### 13. Benchmarks Determine Viability
Agent task benchmarks, not storage benchmarks alone. Measure: task success, tool calls avoided, tokens saved, source reads avoided.

## Confidence Levels

All research findings must distinguish:

| Level | Meaning |
|-------|---------|
| FACT | Verified by primary source, reproducible |
| OBSERVATION | Directly observed, reproducible |
| HYPOTHESIS | Proposed explanation, requires validation |
| INFERENCE | Deduced from evidence, marked as such |
| OPEN QUESTION | Explicitly unknown, needs research |

## What This Means in Practice

- No "Prime uses SQLite because it's fast" without evidence
- No "Prime uses protobuf because it's standard" without comparison
- No "Prime's graph model is X" without research justification
- Every major design area has a research document with alternatives and tradeoffs
- The specification is a living document reflecting current evidence

## Next

- [Roadmap](./roadmap.md)
- [Research Overview](../research/agent-architecture.md)
- [Research: Codebase Knowledge](../research/codebase-knowledge.md)