# Prime Retrieval Reference

## Retrieval Strategy

Prime answers agent questions through **structured retrieval operations** over a derived knowledge artifact. The artifact is built once from source and queried many times.

---

## Retrieval Operations

### `prime.find(symbol, opts?)`
**Purpose**: Locate a symbol by name (exact or fuzzy).

| Input | Type | Required |
|-------|------|----------|
| `symbol` | string | Yes |
| `opts.fuzzy` | boolean | No (default: false) |
| `opts.kind` | SymbolKind | No |
| `opts.limit` | number | No (default: 20) |

**Returns**: `EntitySummary[]`

**Failure conditions**: No matches, or symbol is ambiguous.

---

### `prime.lookup(qualified_name, opts?)`
**Purpose**: Get full definition + metadata for a known qualified name.

| Input | Type | Required |
|-------|------|----------|
| `qualified_name` | string | Yes |
| `opts.include_docs` | boolean | No (default: false) |

**Returns**: `EntitySummary` with full signature, documentation, location.

**Failure conditions**: Not found.

---

### `prime.references(symbol, opts?)`
**Purpose**: Find all references to a symbol.

| Input | Type | Required |
|-------|------|----------|
| `symbol` | string \| EntityId | Yes |
| `opts.kind` | RelationKind | No (default: References) |
| `opts.transitive` | boolean | No (default: false) |
| `opts.limit` | number | No |

**Returns**: `Reference[]` with file, location, context.

**Failure conditions**: Symbol not found, no references.

---

### `prime.callers(symbol, opts?)`
**Purpose**: Who calls this function?

| Input | Type | Required |
|-------|------|----------|
| `symbol` | string \| EntityId | Yes |
| `opts.transitive` | boolean | No (default: false) |
| `opts.depth` | number | No (default: 1) |

**Returns**: `Caller[]` with caller symbol, location, call context.

---

### `prime.callees(symbol, opts?)`
**Purpose**: What does this function call?

| Input | Type | Required |
|-------|------|----------|
| `symbol` | string \| EntityId | Yes |
| `opts.transitive` | boolean | No (default: false) |
| `opts.depth` | number | No (default: 1) |

**Returns**: `Callee[]` with callee symbol, location, call context.

---

### `prime.implementations(interface, opts?)`
**Purpose**: Find concrete implementations of an interface/trait.

| Input | Type | Required |
|-------|------|----------|
| `interface` | string \| EntityId | Yes |
| `opts.include_partial` | boolean | No (default: false) |

**Returns**: `Implementation[]` with implementor, location, confidence.

---

### `prime.implementers(trait, opts?)`
**Purpose**: Find types implementing a trait/protocol.

| Input | Type | Required |
|-------|------|----------|
| `trait` | string \| EntityId | Yes |
| `opts.include_generic` | boolean | No (default: true) |

**Returns**: `Implementation[]`

---

### `prime.dependencies(symbol, opts?)`
**Purpose**: What does this symbol depend on?

| Input | Type | Required |
|-------|------|----------|
| `symbol` | string \| EntityId | Yes |
| `opts.transitive` | boolean | No (default: false) |
| `opts.kind` | RelationKind[] | No |

**Returns**: `Dependency[]` with target, kind, confidence.

---

### `prime.dependents(symbol, opts?)`
**Purpose**: What depends on this symbol?

| Input | Type | Required |
|-------|------|----------|
| `symbol` | string \| EntityId | Yes |
| `opts.transitive` | boolean | No (default: false) |
| `opts.kind` | RelationKind[] | No |

**Returns**: `Dependent[]`

---

### `prime.impact(symbol, opts?)`
**Purpose**: What breaks if this symbol changes?

| Input | Type | Required |
|-------|------|----------|
| `symbol` | string \| EntityId | Yes |
| `opts.depth` | number | No (default: 3) |
| `opts.include_tests` | boolean | No (default: true) |

**Returns**: `ImpactReport` with affected symbols, blast radius, risk score.

---

### `prime.context(symbol, opts?)`
**Purpose**: Minimal useful knowledge neighborhood for an entity.

| Input | Type | Required |
|-------|------|----------|
| `symbol` | string \| EntityId | Yes |
| `opts.depth` | number | No (default: 1) |
| `opts.include_relations` | boolean | No (default: true) |
| `opts.max_tokens` | number | No (default: 8192) |

**Returns**: `ContextPackage` with entity, relations, dependents, dependencies, callers, callees within token budget.

---

### `prime.architecture(module, opts?)`
**Purpose**: Module boundaries, layering, architectural patterns.

| Input | Type | Required |
|-------|------|----------|
| `module` | string \| EntityId | Yes |
| `opts.depth` | number | No (default: 2) |

**Returns**: `ArchitectureView` with layers, boundaries, violations.

---

### `prime.tests(symbol, opts?)`
**Purpose**: Test coverage, related tests.

| Input | Type | Required |
|-------|------|----------|
| `symbol` | string \| EntityId | Yes |
| `opts.include_integration` | boolean | No (default: true) |

**Returns**: `TestInfo[]` with test name, type, location, status.

---

### `prime.configuration(symbol, opts?)`
**Purpose**: Configuration affecting symbol behavior.

| Input | Type | Required |
|-------|------|----------|
| `symbol` | string \| EntityId | Yes |

**Returns**: `ConfigInfo[]` with config key, value, source, effect.

---

### `prime.contracts(symbol, opts?)`
**Purpose**: Interfaces, pre/post conditions, invariants.

| Input | Type | Required |
|-------|------|----------|
| `symbol` | string \| EntityId | Yes |

**Returns**: `ContractInfo[]` with contract type, description, enforcement.

---

### `prime.architecture(module, opts?)`
**Purpose**: Module boundaries, layering, architectural patterns.

| Input | Type | Required |
|-------|------|----------|
| `module` | string \| EntityId | Yes |
| `opts.depth` | number | No (default: 2) |

**Returns**: `ArchitectureView` with layers, boundaries, violations.

---

### `prime.search(query, opts?)`
**Purpose**: Semantic/lexical search across knowledge.

| Input | Type | Required |
|-------|------|----------|
| `query` | string | Yes |
| `opts.mode` | "lexical" \| "semantic" \| "hybrid" | No (default: "hybrid") |
| `opts.limit` | number | No (default: 20) |
| `opts.kinds` | SymbolKind[] | No |

**Returns**: `SearchResult[]` with score, entity, snippet.

---

### `prime.slice(entity_id, opts?)`
**Purpose**: Compact AI-optimized graph slice for an entity.

| Input | Type | Required |
|-------|------|----------|
| `entity_id` | EntityId | Yes |
| `opts.radius` | number | No (default: 2) |
| `opts.max_nodes` | number | No (default: 100) |

**Returns**: `GraphSlice` with nodes, edges, subgraph.

---

## Response Formats

### EntitySummary
```json
{
  "id": "entity_id",
  "kind": "Function",
  "name": "login",
  "qualified_name": "AuthService.login",
  "language": "TypeScript",
  "range": {"start": {"line": 10, "column": 5}, "end": {"line": 45, "column": 1}},
  "signature": "login(email: string, password: string): Promise<Session>",
  "documentation": "Authenticates user...",
  "confidence": "exact",
  "relation_counts": {
    "Calls": 3,
    "References": 12,
    "DependsOn": 4
  }
}
```

### RelationSummary
```json
{
  "kind": "Calls",
  "target_id": "entity_id",
  "target_name": "UserRepository.findByEmail",
  "target_kind": "Method",
  "confidence": "exact"
}
```

### RelationKind
```
Contains, PartOf,
Extends, Implements, Inherits,
DependsOn, Imports, Requires,
Calls, References, Reads, Writes,
Returns, ParameterOf, TypeOf, GenericArgOf,
Overrides, Overloads,
Exports, ReExports
```

### Confidence Levels
- `exact` — Verified by primary source
- `observation` — Directly observed
- `hypothesis` — Proposed, needs validation
- `inference` — Deduced from evidence
- `open_question` — Explicitly unknown

---

## Provenance

Every fact carries provenance:
```json
{
  "kind": "declared",
  "source": "src/auth/AuthService.ts:10-45",
  "detail": "Dependencies section"
}
```
Types: `declared` (from AGENTS.md), `discovered` (from analysis), `inferred` (suggested by ACC), `memory` (from .acc-memory.md).

---

## Response Envelope
```json
{
  "status": "complete" | "partial" | "unknown",
  "source_required": boolean,
  "data": { ... },
  "missing": ["exact implementation behavior"],
  "confidence": "exact" | "observation" | "hypothesis" | "inference" | "open_question"
}
```

---

## Query Options

### QueryOptions
```typescript
interface QueryOptions {
  max_results?: number;           // default: 50
  include_relations?: boolean;    // default: true
  relation_kinds?: RelationKind[]; // filter
  max_depth: number;              // default: 2
  min_confidence: Confidence;     // default: "medium"
  include_documentation: boolean; // default: false
  include_signatures: boolean;    // default: true
  token_budget: number;           // default: 8192
}
```

### Default Profiles
```typescript
QueryOptions.for_agent()     // token_budget: 8192, depth: 2
QueryOptions.for_exploration() // token_budget: 32768, depth: 3
```

---

## Progressive Context Builder

```typescript
const builder = engine.context_builder(token_budget);
builder.add_symbol("AuthService.login");
builder.expand_context("AuthService.login", depth=2);
const context = builder.get_included();
```

---

## Streaming Query

```typescript
const stream = engine.streaming(opts);
stream.search("AuthService").forEach(r => ...);
```

---

## Confidence Handling

```typescript
if (result.confidence === "inference") {
  // flag for human review
}
if (result.confidence === "unknown") {
  // must escalate to source
}
```

---

## Token Efficiency

Prime optimizes for **minimum useful representation**:

- Structured output > prose
- Relationships over text
- Confidence/provenance included
- Progressive disclosure (start minimal, expand on demand)
- Token budget enforced per query

---

## Failure Modes

| Status | Meaning | Agent Action |
|--------|---------|--------------|
| `complete` | Fully answered | Use result |
| `partial` | Some gaps | Use known facts, escalate gaps |
| `unknown` | Cannot answer | Must escalate to source |

Never make the agent guess whether Prime is complete.

---

## Source Escalation

```json
{
  "status": "partial",
  "source_required": true,
  "missing": ["exact implementation behavior"],
  "escalation_reason": "Prime covers call graph but not exact error message logic"
}
```

The agent should:
1. Use Prime for what it knows
2. Read source only for missing pieces
3. Record escalation reason in `.acc-memory.md`

---

## Configuration

Retrieval behavior configured in `.acc/config/config.yaml`:
```yaml
language_analyzers:
  rust: true
  typescript: true
  python: true
  # ...

ignore:
  - "target/"
  - "node_modules/"

diagnostics:
  warn_only: ["ACC010"]  # downgrade template placeholder errors

engine:
  trigger:
    mode: commits
    threshold: 3
```

---

## Related Documents

- Skill: `../SKILL.md`
- ACC configuration: `.acc/config/`
- Research specifications: `SPECS/`
- Research methodology: `.acc/config/workflows/research.md`