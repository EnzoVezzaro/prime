# OpenWiki — Agent-Generated Semantic Knowledge + Incremental Maintenance

## What It Is

OpenWiki (by LangChain) is a CLI that reads a codebase, has an agent synthesize documentation, keeps that documentation updated from Git changes, and exposes the result as a linked Markdown wiki. It now also emits Open Knowledge Format (OKF v0.2), where concepts are Markdown documents with typed front matter and relationships are represented by links.

Repository: https://github.com/langchain-ai/openwiki

## Key Architectural Ideas

### 1. Knowledge Lifecycle (not one-shot generation)

OpenWiki doesn't just generate documentation once. It uses Git changes to determine what changed and updates the relevant knowledge units.

```
commit N
   │
   ▼
changed symbols/files
   │
   ▼
affected knowledge units
   │
   ▼
incremental re-derivation
   │
   ▼
new wiki snapshot
```

This is analogous to what Prime should investigate for incremental semantic derivation.

**FACT:** OpenWiki explicitly maintains knowledge through code changes via CI integration (GitHub Actions, GitLab CI, Bitbucket Pipelines).

**RELEVANCE TO PRIME:** Prime's planned "incremental analysis and invalidation" could follow a similar lifecycle pattern: detect changed symbols → identify affected knowledge → re-derive only affected facts → update the artifact.

### 2. Grounded Claims (Evidence-Backed Propositions)

OpenWiki tracks material propositions behind factual pages — not just when a Markdown file was last generated. Claims cover: behavior, responsibilities, architecture, data flow, invariants, failure semantics, configuration, and security boundaries.

Each claim points to exact repository evidence such as `repo://src/server.ts#L40-L82`, with the evidence version OpenWiki observed when the claim was established.

Before an update, OpenWiki checks evidence versions. If source lines changed or disappeared, the affected claim becomes stale or unresolved.

```
claim: "AuthService handles JWT token validation"
evidence: repo://src/auth.ts#L40-L82
evidence_version: abc123 (commit hash)
status: verified | stale | unresolved
```

**FACT:** Grounded Claims live under `openwiki/.claims/` as structured sidecars alongside the Markdown.

**RELEVANCE TO PRIME:** Prime already has `AgentProvenance` and `Confidence` levels (exact, derived, inferred, unknown). OpenWiki's Grounded Claims model suggests Prime should track not just confidence but also the *source evidence* and *evidence version* for each fact, enabling incremental invalidation.

### 3. OKF v0.2 (Open Knowledge Format)

Every concept document carries YAML front matter with a non-empty `type`. Standard Markdown links between concept documents express their relationships.

```
---
type: concept
generated:
  by: openwiki/1.0.0
  at: 2026-08-21T12:00:00Z
verified:
  by: openwiki/1.0.0
  at: 2026-08-21T12:00:00Z
sources:
  - repo://src/auth.ts#L40-L82
---

# AuthService

The AuthService handles JWT token validation...
```

**FACT:** OKF v0.2 is a Google-backed standard with validated provenance, trust, and lifecycle metadata.

**RELEVANCE TO PRIME:** Prime's `PrimeEnvelope<T>` response envelope is conceptually similar but at the query level. OKF suggests Prime's artifact itself could use a format where each knowledge unit carries typed metadata, provenance, and lifecycle state — rather than just storing raw entities and relations.

### 4. Agent-Readable Knowledge

OpenWiki explicitly modifies `AGENTS.md` and `CLAUDE.md` so coding agents know the wiki exists and can use it when they need repository context.

**HYPOTHESIS:** This creates an interesting benchmark opportunity:

```
agent + source only
agent + source + OpenWiki
agent + source + Prime
agent + source + Prime + OpenWiki
```

> Does a compact machine representation (Prime) outperform human-readable generated documentation (OpenWiki)?

**OPEN QUESTION:** These might be complementary rather than competing — Prime for structural/traversal queries, OpenWiki for semantic/behavioral queries.

### 5. Two-Level Knowledge Architecture

OpenWiki separates:
- **Deterministic setup/finalization** — file structure, indexes, provenance metadata
- **Agent-driven synthesis** — the actual documentation content

Prime should investigate a similar split:

```
DETERMINISTIC LAYER (what Prime already does):
  AST → symbols → references → relationships → indexes

AGENT-DRIVEN LAYER (what OpenWiki adds):
  behavior → intent → architecture → documentation → context
```

The deterministic layer is fast, reproducible, and compact. The agent layer adds semantic depth that static analysis cannot provide.

## Comparison with Prime

| Dimension | OpenWiki | Prime |
|-----------|----------|-------|
| Generation method | Agent (LLM) | Static analysis |
| Output format | Markdown + OKF | Binary artifact + PrimeEnvelope |
| Incremental updates | Yes (Git-based) | Planned |
| Evidence tracking | Grounded Claims | AgentProvenance |
| Confidence model | verified/stale/unresolved | exact/derived/inferred/unknown |
| Primary consumer | Agent via AGENTS.md | Agent via MCP tools |
| Representation | Human-readable wiki | Machine-optimized index |
| Query model | Navigate links | Lookup/search/traverse |

## What Prime Should Steal from OpenWiki

| OpenWiki Idea | Prime Adaptation | Keep? |
|---------------|------------------|-------|
| Grounded Claims | Evidence + version tracking per fact | **Yes** |
| Incremental maintenance via Git | Derive affected knowledge from changed files | **Yes** |
| Evidence versioning | Content hash per fact for invalidation | **Yes** |
| Two-level architecture (deterministic + agent) | Deterministic base + optional agent enrichment | **Yes** |
| AGENTS.md integration | MCP-based agent discovery | **Maybe** |
| OKF format | Typed knowledge units with lifecycle metadata | **Maybe** |
| Human-readable wiki output | Not Prime's goal | **No** |
| LLM-based synthesis | Static analysis is preferred for code | **No** |

## What Prime Should NOT Steal

1. **LLM-based synthesis as the primary derivation method** — Prime should use deterministic/static analysis first. The entire point is that parsers, compilers, and LSP give us what LLMs approximate.

2. **Markdown as the artifact format** — Prime's binary artifact with specialized indexes will always be more compact and faster to query than Markdown files.

3. **The "documentation" framing** — Prime is not documentation. It's a derived knowledge representation optimized for agent retrieval.

## Open Questions

1. Should Prime's artifact carry per-fact evidence versions like Grounded Claims?
2. Could Prime use a content-addressed fact store (Merkle DAG) to efficiently detect which facts need re-derivation after a commit?
3. Is the optimal architecture: Prime (deterministic) + OpenWiki (semantic) or a single system that does both?
4. Should Prime emit a human-readable view of its knowledge (like OpenWiki's visualizer) for debugging and trust verification?
5. Can Prime's incremental derivation be modeled as a Git-aware knowledge compiler?
