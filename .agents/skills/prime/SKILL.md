---
name: prime
description: Use Prime as the primary codebase knowledge interface via MCP. Query Prime before searching, reading, or reconstructing repository structure. Use Prime for symbols, relationships, dependencies, impact, architecture, and context. Escalate to source only when Prime cannot answer the required question.
---

# Prime Skill — Agent Codebase Knowledge Interface

**Prime is the primary codebase knowledge interface.**

Do not search files, grep, or read source to answer codebase questions unless Prime cannot answer.

Prime exposes 7 semantic tools via MCP (Model Context Protocol):

| Tool | Description |
|------|-------------|
| `prime_search` | Search entities by keyword |
| `prime_lookup` | Look up entity by qualified name |
| `prime_context` | Get knowledge neighborhood (deps, callers, callees) |
| `prime_relationships` | Get relationships across dimensions |
| `prime_dependencies` | Get dependency graph |
| `prime_impact` | Analyze impact of changes |
| `prime_architecture` | Get architecture overview |

---

## Prime Workflow

### Before exploration

**Do not immediately inspect source.**

First determine whether Prime can answer the question.

```
Question → Prime → Answer
                │
                ├── Complete → Answer
                ├── Partial → Prime answer + escalation option
                └── Unknown → Source escalation
```

---

## Source Escalation Policy

**Source is an escalation path, not the default.**

Every Prime tool returns a `PrimeEnvelope` with:

| Field | Meaning |
|-------|---------|
| `status` | `complete`, `partial`, `unknown`, `error` |
| `source_required` | Whether source code access is needed |
| `coverage` | Knowledge completeness (0.0–1.0) |
| `missing` | What Prime knows is missing |
| `provenance` | How facts were derived |
| `warnings` | Issues with the result |

Only escalate to source when:

| Condition | Action |
|-----------|--------|
| `status: unknown` | Read source for exact implementation |
| `status: partial` | Use Prime for known facts, then read source for gaps |
| `coverage < 0.8` | Prime may be missing details |
| Task requires exact implementation | Use Prime for context, then read source |

**Never** read source first. Never read source when Prime can answer.

---

## Agent Confidence

Prime returns confidence with every entity:

| Level | Meaning |
|-------|---------|
| `exact` | Directly observable in source; verified |
| `derived` | Statically derived with high certainty |
| `inferred` | Inferred from patterns; may be incomplete |
| `unknown` | Cannot determine confidence |

**Never** treat inferred knowledge as exact. Always check confidence.

---

## Using the MCP Server

Start the Prime MCP server:

```bash
prime serve
```

The server runs on stdio transport and exposes all 7 tools.

---

## Using the CLI

```bash
# Build knowledge graph
prime build --root /path/to/project

# Inspect an entity with agent envelope
prime inspect AuthService.login --format json

# Query the knowledge graph
prime query "AuthService" --type search --format json

# Start MCP server
prime serve

# Run benchmark
prime benchmark
```

---

## Installation

This skill is installed via ACC:

```bash
acc skill install prime
# or
npx skills add <repo> --skill prime
```

---

## References

- Retrieval protocol: `references/retrieval.md`
- ACC configuration: `.acc/config/`
- Research specifications: `SPECS/`
- Agent-Native Interface spec: `SPECS/agent-native-interface.md`
