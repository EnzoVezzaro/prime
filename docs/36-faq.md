---
title: FAQ
---

# Frequently Asked Questions

## General

### What is Prime?
Prime is a research project investigating how a codebase can be transformed into a compact, language-agnostic knowledge artifact optimized for agent retrieval.

### Is Prime a product?
No. Prime is research. No final format, graph model, storage design, retrieval API, or compression algorithm exists yet.

### What problem does Prime solve?
Agents repeatedly reconstruct codebase knowledge from source files. Prime derives that knowledge once so agents can answer questions without source retrieval when possible.

### How is Prime different from SCIP/LSIF?
SCIP/LSIF are indexing protocols for language servers. Prime is an agent-optimized knowledge representation with semantic tools, provenance, confidence, and source escalation.

### How is Prime different from a graph database?
Prime is not a database. It's a derived artifact. The physical storage is an implementation detail decided by research.

## Technical

### What languages does Prime support?
Current implementation: TypeScript, JavaScript, Python, Rust, Go, Java, C, C++. More under research.

### Does Prime replace source code?
No. The source remains authoritative. Prime is the derived fast path.

### How does Prime handle dynamic languages?
Confidence levels distinguish `exact` (static) from `inferred` (dynamic dispatch, reflection). Source escalation handles the rest.

### What is the artifact format?
Undecided. Research evaluates: custom binary, columnar, content-addressed blocks, succinct structures.

### How does incremental update work?
Planned: Merkle tree of source → invalidate affected entities → re-derive → update artifact.

### Can Prime run on large codebases (1M+ files)?
Research target. Requires streaming, parallel analysis, partial loading, memory mapping.

## Agent Integration

### How do agents use Prime?
Via MCP (Model Context Protocol) — 7 semantic tools: `prime_search`, `prime_lookup`, `prime_context`, `prime_relationships`, `prime_dependencies`, `prime_impact`, `prime_architecture`.

### What is the envelope?
Every tool returns `PrimeEnvelope<T>` with `status`, `coverage`, `source_required`, `provenance`, `warnings`, `result`.

### What if Prime can't answer?
`source_required: true` + `escalation_hints` → agent fetches targeted source.

### Does Prime work with any agent?
Yes. MCP is agent-agnostic. Works with Claude, Cursor, Codex, OpenCode, Gemini, custom agents.

## Research

### Why so many research areas?
The optimal solution may come from outside traditional code tooling (information theory, succinct structures, cryptography, distributed systems).

### How do I contribute?
See `CONTRIBUTING.md`. Useful: papers, technical references, implementation analysis, benchmarks, experiments, counterexamples.

### Where are research findings?
In `research/` (external knowledge) and `docs/` (synthesized conclusions).

### What confidence levels exist?
`exact` (verified), `derived` (computed), `inferred` (heuristic), `unknown` (no evidence).

## Getting Started

### How do I run Prime?
```bash
cd prime-rs
cargo build --release --workspace
cargo run --bin prime -- build --root /path/to/project --storage /path/to/storage
cargo run --bin prime -- query "AuthService.login"
```

### How do I run benchmarks?
```bash
cd prime-rs
cargo bench -p prime-bench
```

### Where is the documentation?
This site. Source in `docs/`. Built with VitePress.

## Project

### Who maintains Prime?
Enzo Vezzaro. Open research, MIT licensed.

### What is ACC?
Agent Code Context — Prime's sibling project for project-level agent context. Different layer.

### Is Prime affiliated with any company?
No. Independent open research.

### Can I use Prime in production?
Research stage. Not recommended for production use.