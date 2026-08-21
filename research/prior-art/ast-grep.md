# ast-grep Prior Art Analysis

## Confidence Legend

| Label | Meaning |
|-------|---------|
| **FACT** | Verified by primary source (source code, official docs, technical specs) |
| **OBSERVATION** | Directly observed from implementation artifacts |
| **HYPOTHESIS** | Proposed explanation requiring validation |
| **INFERENCE** | Deduced from evidence, marked as such |
| **OPEN QUESTION** | Explicitly unknown, needs research |

---

## Executive Summary

ast-grep (ast-grep/ast-grep) is a Rust-based structural search, lint, and rewrite tool built on Tree-sitter. It exposes a **pattern language** (code-as-pattern with meta-variables `$VAR`, `$$$MULTI`), **YAML rule configuration** (atomic, relational, composite rules), **transformation pipeline** (substring, replace, convert, rewrite), and **programmatic API** (jQuery-like AST traversal). Code-Graph-RAG integrates ast-grep for structural findings (`Pattern`, `CodeSmell`, `SecurityIssue` nodes) and Ruby support via pluggable YAML patterns. This analysis evaluates whether **structural patterns could become a Prime query primitive**.

---

## Core Architecture

**FACT** — From [ast-grep.github.io](https://ast-grep.github.io/) and [advanced/how-ast-grep-works.md](https://ast-grep.github.io/advanced/how-ast-grep-works.html)

```
Source Code → Tree-sitter Parser → AST → Pattern Matcher → Results
                                    ↓
                              Rewrite Engine → Transformed Code
```

- **Parsing**: Tree-sitter (50+ languages via dynamic loading)
- **Matching**: Custom tree pattern matching algorithm (Rust, multi-core)
- **Rewriting**: Source-to-source transformation preserving formatting
- **Interfaces**: CLI, YAML rules, Node.js/Python/Rust APIs, MCP-compatible JSON output

**FACT** — 20+ supported languages: C, C#, Go, Java, JavaScript, TypeScript, TSX, Kotlin, Python, Rust, Ruby, HTML, YAML, Dart, Lua, PHP, Scala, Swift, etc. Custom languages loadable via dynamic Tree-sitter parser `.so`/`.dylib`/`.dll`.

---

## Pattern Syntax

**FACT** — From [guide/pattern-syntax.md](https://ast-grep.github.io/guide/pattern-syntax.html)

### Meta-Variables
- `$META` (uppercase + underscore + digits) — matches **single** AST node
- `$$$MULTI` — matches **zero or more** AST nodes (arguments, parameters, statements)
- `$_NON_CAPTURING` — matches but doesn't capture (optimization)

### Pattern Matching Semantics
```javascript
// Pattern: console.log($GREETING)
console.log('Hello World')        // MATCH: $GREETING = 'Hello World'
console.log(a, b)                 // NO MATCH: too many args
console.log()                     // NO MATCH: missing arg
```

### Capture & Reuse
```javascript
// Pattern: $A == $A
a == a              // MATCH: $A = a
1 + 1 == 1 + 1      // MATCH: $A = 1 + 1
a == b              // NO MATCH: different nodes
```

### Pattern Object (for ambiguous syntax)
```yaml
pattern:
  selector: field_definition    # Target node kind
  context: class A { $FIELD = $INIT }  # Disambiguating context
```

### Strictness Levels (most → least strict)
| Level | Behavior |
|-------|----------|
| `cst` | All nodes matched (including punctuation, whitespace) |
| `smart` | Default: named nodes matched, unnamed skipped |
| `ast` | Only named AST nodes matched |
| `relaxed` | Named nodes + comments ignored |
| `signature` | Only node kinds matched (structure only) |

---

## Rule System (YAML Configuration)

**FACT** — From [guide/rule-config/atomic-rule.md](https://ast-grep.github.io/guide/rule-config/atomic-rule.html), [relational-rule.md](https://ast-grep.github.io/guide/rule-config/relational-rule.html), [composite-rule.md](https://ast-grep.github.io/guide/rule-config/composite-rule.html)

### Atomic Rules (single node match)
| Rule | Purpose |
|------|---------|
| `pattern` | Structural pattern with meta-variables |
| `kind` | Match by Tree-sitter node kind (e.g., `function_declaration`) |
| `regex` | Match node text against Rust regex |
| `nthChild` | Match by sibling position (CSS `nth-child` syntax) |
| `range` | Match by source location (line/column) |

### Relational Rules (node relationships)
| Rule | Purpose |
|------|---------|
| `has` | Descendant matches sub-rule |
| `inside` | Ancestor matches sub-rule |
| `follows` / `precedes` | Sibling order |
| `any` / `all` / `none` | Logical composition |

### Composite Rules
```yaml
rule:
  all:                    # Logical AND
    - pattern: $FUNC($$$ARGS)
    - has:
        pattern: return $VAL
        stopBy: end       # Stop at function end
  any:                    # Logical OR
    - kind: async_function
    - kind: function_declaration
```

### Utility Rules
- `stopBy`: Limit traversal scope (e.g., `end`, `nextSibling`)

---

## Rewrite & Transformation Pipeline

**FACT** — From [guide/rewrite/transform.md](https://ast-grep.github.io/guide/rewrite/transform.html) and [guide/rewrite/rewriter.md](https://ast-grep.github.io/guide/rewrite/rewriter.html)

### Fix Template
```yaml
fix: '[$LIST]'           # Use captured meta-variable in replacement
```

### Transformations (string operations on captured variables)
| Transform | Purpose | Example |
|-----------|---------|---------|
| `replace` | Regex replace with capture groups | `replace: source=$OLD, replace=debug(?<REST>.*), by=release$REST` |
| `substring` | Slice by character indices | `substring: source=$GEN, startChar=1, endChar=-1` |
| `convert` | Case conversion | `convert: source=$NAME, toCase=camelCase` |
| `rewrite` | Recursive sub-rewrite | `rewrite: source=$BODY, rules=[...]` |

### Chaining Transforms
```yaml
transform:
  KEBABED: convert($OLD_FN, toCase=kebabCase)
  RELEASED: replace($KEBABED, replace='-debug', by='-release')
  UNKEBABED: convert($RELEASED, toCase=camelCase)
fix: $UNKEBABED($$$ARGS)
```

### Conditional Text (DasSurma Trick)
```yaml
transform:
  MAYBE_COMMA: replace($$$ARGS, replace='^.+', by=', ')
fix: $FUNC(new_arg$MAYBE_COMMA$$$ARGS)
```

---

## Pattern Matching Algorithm

**FACT** — From [advanced/pattern-parse.md](https://ast-grep.github.io/advanced/pattern-parse.html) and [advanced/match-algorithm.md](https://ast-grep.github.io/advanced/match-algorithm.html)

1. **Pattern Parsing**: User pattern → Tree-sitter parse → Pattern AST
2. **Meta-variable Extraction**: Identify `$VAR` and `$$$MULTI` nodes
3. **Matching**: For each target AST node:
   - Structural equality on named nodes
   - Meta-variable binding (single or multi)
   - Strictness filtering
   - Relational constraint checking
4. **Optimization**: Rules with `kind` filter candidate nodes before pattern matching

**OBSERVATION** — Matching is **single-tree, single-pass** per file. No cross-file analysis. Multi-core parallelism at file level.

---

## Integration with Code-Graph-RAG

**FACT** — From Code-Graph-RAG `docs/architecture/graph-schema.md` and `NEWS.md`

- **Ruby support**: Entire language added via **single YAML pattern file** (ast-grep tier) emitting `Module`, `Function`, `Class` nodes + `IMPORTS` edges — no hand-written parser
- **Structural findings**: `Pattern`, `CodeSmell`, `SecurityIssue` nodes created from ast-grep rule matches; edges `IMPLEMENTS_PATTERN`, `HAS_SMELL`, `HAS_VULNERABILITY`
- **Agent tools**: Structural search/replace exposed as MCP tools

**INFERENCE** — ast-grep's YAML rules act as a **declarative extraction language** for syntactic patterns. Code-Graph-RAG treats findings as first-class graph entities.

---

## Programmatic API

**FACT** — From [guide/api-usage/js-api.md](https://ast-grep.github.io/guide/api-usage/js-api.html) and [reference/api.md](https://ast-grep.github.io/reference/api.html)

```javascript
import { parse } from '@ast-grep/napi';
const sg = parse(code, 'typescript');
const matches = sg.root().findAll({ pattern: 'console.log($MSG)' });
for (const m of matches) {
  console.log(m.getMatch('MSG').text());
}
```

- **jQuery-like**: `root()`, `find()`, `findAll()`, `parent()`, `children()`, `siblings()`
- **Type-safe NAPI** (opt-in): TypeScript definitions for node kinds per language
- **Batch API**: `ast-grep scan --json` for programmatic consumption

---

## What Is CHEAPLY Derivable from ast-grep Patterns

| Information | Derivation Cost | Evidence |
|-------------|----------------|----------|
| **Syntactic pattern matches** | O(file size) per pattern | FACT: Single-pass tree match |
| **Parameter lists** (`$$$ARGS`) | O(1) per call site | FACT: Multi-meta-variable |
| **Function signatures** (name + params) | O(functions) | FACT: Pattern `function $FUNC($$$ARGS)` |
| **Import statements** | O(imports) | FACT: Pattern `import $MOD from '$PATH'` |
| **Class/method definitions** | O(definitions) | FACT: Pattern `class $NAME { $$$ }` |
| **Control flow patterns** | O(nodes) | FACT: `if ($COND) { $$$ }` |
| **Annotation/decorator patterns** | O(annotations) | FACT: `@$DECORATOR class $NAME` |
| **String/template interpolation** | O(templates) | FACT: Pattern `` `text ${$VAR}` `` |
| **Rewrite previews** | O(matches) | FACT: Fix template + transforms |

---

## What REQUIRES Analysis Beyond ast-grep Patterns

| Information | Why Patterns Cannot Provide |
|-------------|----------------------------|
| **Resolved symbol identity** | Patterns match syntax, not cross-file symbol tables |
| **Call graph edges** | `foo()` — which `foo`? Patterns can't resolve overloads |
| **Type information** | No type inference in pattern language |
| **Inheritance hierarchy** | `class A extends B` — which `B`? Import resolution needed |
| **Data flow** | Patterns are structural, not value-tracking |
| **Dead code** | Requires reachability from entry points |
| **Cross-file impact** | Patterns operate per-file |
| **Framework-specific semantics** | DI containers, ORM, routing — need semantic models |

---

## What Prime Should Borrow

| Feature | Rationale | Evidence |
|---------|-----------|----------|
| **Pattern-as-query primitive** | Declarative structural queries composable with semantic tools | FACT: YAML rules = portable pattern specs |
| **Meta-variable capture (`$VAR`, `$$$MULTI`)** | Natural syntax for "find X where..." agent queries | FACT: Pattern syntax isomorphic to code |
| **Transformation pipeline** | Automated codemod/refactoring as agent capability | FACT: `replace`, `convert`, `rewrite` chain |
| **Strictness levels** | Tunable precision/recall for structural queries | FACT: `cst` → `signature` spectrum |
| **Relational rules (`has`, `inside`, `follows`)** | Structural context without full semantic analysis | FACT: Ancestor/descendant/sibling constraints |
| **YAML rule portability** | Shareable, versionable lint/pattern catalogs | FACT: Vercel Turbo, Vue Macros, CodeRabbit use this |
| **Dynamic language loading** | Add languages without recompiling Prime | FACT: Tree-sitter `.so` + YAML patterns |
| **Pattern object (`context` + `selector`)** | Disambiguate syntactic constructs | FACT: Solves `field_definition` vs `assignment_expression` |
| **Conditional rewrite (DasSurma trick)** | Context-sensitive formatting in transforms | FACT: `replace` on `$$$MULTI` for commas |

---

## What Prime Should NOT Borrow

| Feature | Rationale | Evidence |
|---------|-----------|----------|
| **Per-file matching only** | Prime needs cross-file semantic queries | INFERENCE: ast-grep has no cross-file symbol resolution |
| **YAML as primary query API** | Prime's agent API is 7 fixed semantic tools | FACT: PrimeEnvelope<T> vs. ad-hoc rule execution |
| **Source-to-source rewrite as core** | Prime is read-only knowledge representation | INFERENCE: Prime focuses on retrieval, not mutation |
| **Tree-sitter CST as internal representation** | Prime targets compact binary/mmap format | INFERENCE: Different storage goals |

---

## Open Questions

1. **OPEN QUESTION**: Could Prime adopt **ast-grep patterns as a query primitive** alongside its 7 semantic tools? E.g., `prime query --pattern "async function $NAME($$$ARGS) { await $$$ }"` returning `PrimeEnvelope<PatternMatch[]>`.

2. **OPEN QUESTION**: ast-grep's `kind` rule uses Tree-sitter node kinds directly. Prime's entity types are language-agnostic. Can we define a **mapping from TS node kinds → Prime entity types** that's queryable via pattern?

3. **OPEN QUESTION**: The `transform` pipeline (substring, replace, convert, rewrite) — could Prime's **impact analysis** use similar transformations to simulate "what if this function signature changes?" without full semantic analysis?

4. **OPEN QUESTION**: Code-Graph-RAG uses ast-grep for **Ruby support without a parser**. Could Prime adopt a similar "pluggable pattern tier" for languages lacking Tree-sitter grammars or semantic analyzers?

5. **OPEN QUESTION**: ast-grep's `nthChild` and `range` rules — could Prime's **location-aware queries** (e.g., "functions in file X between lines 100-200") use similar primitives?

6. **OPEN QUESTION**: Pattern strictness levels (`cst` → `signature`) — should Prime's **confidence levels** (`exact`, `derived`, `inferred`, `unknown`) map to strictness? E.g., `signature` match = `inferred`, `cst` match = `exact`?

7. **OPEN QUESTION**: ast-grep's relational rules (`has`, `inside`) are **structural**, not semantic. Prime's `prime_context` tool returns semantic neighborhoods. Could structural relational rules serve as a **fast fallback** when semantic analysis is unavailable?

8. **OPEN QUESTION**: The DasSurma trick (conditional text via `replace` on `$$$MULTI`) — could Prime's **code generation/synthesis** use similar templates for agent-driven edits?

9. **OPEN QUESTION**: ast-grep's multi-core file-parallel architecture — Prime's incremental index update — can pattern matching be incrementally maintained like Tree-sitter CST? Or is full re-scan needed?

10. **OPEN QUESTION**: ast-grep rules can be **composed** (`all`, `any`, `none`). Prime's query tools are fixed. Should Prime expose a **rule composition DSL** for agents to build custom structural queries?

---

## References

- [ast-grep Official Site](https://ast-grep.github.io/) — Primary source for pattern syntax, rules, rewrites
- [Pattern Syntax](https://ast-grep.github.io/guide/pattern-syntax.html) — Primary source for meta-variables, strictness
- [Atomic Rules](https://ast-grep.github.io/guide/rule-config/atomic-rule.html) — Primary source for `pattern`, `kind`, `regex`, `nthChild`, `range`
- [Relational Rules](https://ast-grep.github.io/guide/rule-config/relational-rule.html) — Primary source for `has`, `inside`, `follows`, `precedes`
- [Transformations](https://ast-grep.github.io/guide/rewrite/transform.html) — Primary source for `replace`, `substring`, `convert`, `rewrite`
- [How ast-grep Works](https://ast-grep.github.io/advanced/how-ast-grep-works.html) — Primary source for architecture
- [Pattern Match Algorithm](https://ast-grep.github.io/advanced/match-algorithm.html) — Primary source for matching internals
- [Code-Graph-RAG Graph Schema](https://github.com/vitali87/code-graph-rag/blob/main/docs/architecture/graph-schema.md) — Evidence of ast-grep integration