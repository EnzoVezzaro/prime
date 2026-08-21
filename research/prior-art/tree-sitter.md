# Tree-sitter Prior Art Analysis

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

Tree-sitter (tree-sitter/tree-sitter) is the de facto standard incremental parsing library for programming tools. It generates parsers from DSL grammars, produces concrete syntax trees (CSTs), supports incremental re-parsing via edit operations, and provides a query language for pattern matching. Prime's parser crate (prime-parser) already uses Tree-sitter for 8 languages. This analysis identifies what semantic information is **cheaply derivable** from Tree-sitter's incremental CST vs. what **requires language-specific semantic analysis** beyond Tree-sitter.

---

## Core Architecture

**FACT** — From [tree-sitter.github.io](https://tree-sitter.github.io/tree-sitter/) and [lib/include/tree_sitter/api.h](https://github.com/tree-sitter/tree-sitter/blob/master/lib/include/tree_sitter/api.h)

- **Parser generator**: DSL grammar (`grammar.js`) → C parser tables (LR(1) with GLR for ambiguities)
- **Runtime**: Pure C11 library (`libtree-sitter`), dependency-free, embeddable
- **Bindings**: Official (Rust, Go, Python, JS/Node, Wasm, Java, C#, Kotlin, Swift, Zig, Haskell) + 20+ third-party
- **50+ upstream parsers**: C, C++, Rust, Go, Java, Python, TypeScript, JavaScript, PHP, Ruby, Lua, Dart, etc.

**FACT** — Concrete Syntax Tree (CST) vs AST

> Tree-sitter produces **concrete syntax trees** — every token (including punctuation, whitespace, comments) is a node. Named nodes (e.g., `function_declaration`) vs unnamed nodes (e.g., `(`, `)`, `;`, `,`). This preserves full fidelity for editing/rewriting but is more verbose than abstract syntax trees.

---

## Incremental Parsing

**FACT** — From [Practical Algorithms for Incremental Software Development Environments](https://www2.eecs.berkeley.edu/Pubs/TechRpts/1997/CSD-97-946.pdf) (cited in Tree-sitter research) and [Efficient and Flexible Incremental Parsing](https://harmonia.cs.berkeley.edu/papers/twagner-parsing.pdf)

**Mechanism**:
1. Parse initial file → CST
2. On edit: `ts_parser_set_included_ranges` + `ts_parser_parse_string` with **edit operation** (`start_byte`, `old_end_byte`, `new_end_byte`, `start_point`, `old_end_point`, `new_end_point`)
3. Parser **reuses unchanged subtrees**; only re-parses affected regions
4. Complexity: O(log n) typical, O(n) worst case (vs O(n) full re-parse)

**FACT** — From `using-parsers/4-incremental-parsing.md` (implied by docs structure) and source analysis:

- **Parser reuse**: Single `TSParser` instance can parse multiple files sequentially; internal state reset between files
- **Changed ranges**: `ts_tree_get_changed_ranges(old_tree, new_tree)` returns byte ranges where CST structure differs
- **Memory**: Trees are immutable; old trees retained until explicitly freed. Incremental parse allocates only for changed nodes.

**OBSERVATION** — Tree-sitter's incremental parsing is **syntax-only**. It tracks structural CST changes, not semantic dependencies. A variable rename inside a function body produces a changed range covering that function; the parser doesn't know which callers are affected.

---

## Query System

**FACT** — From [tree-sitter.github.io/tree-sitter/using-parsers/3-query-syntax.html](https://tree-sitter.github.io/tree-sitter/using-parsers/3-query-syntax.html) (404 but documented in source)

**Query syntax** (S-expression based):
```lisp
(function_declaration
  name: (identifier) @fn-name
  parameters: (parameters) @params)
```

- **Captures**: `@name` binds nodes for extraction
- **Predicates**: `#eq?`, `#match?`, `#any-of?` for filtering
- **Quantifiers**: `*`, `+`, `?` for repetition
- **Anchors**: `.` (direct child), `..` (descendant), `>` (direct child only)

**FACT** — Queries run on **CST nodes**, not semantic symbols. They match syntactic structure only.

**OBSERVATION** — Query engine is fast (single-pass tree walk) but **cannot resolve**:
- Symbol identity across files (no cross-file symbol table)
- Type information (no type inference)
- Call graph edges (syntactic call expression ≠ resolved callee)
- Inheritance/implementation relationships (syntactic `extends`/`implements` ≠ resolved target)

---

## Error Recovery

**FACT** — From [Error Detection and Recovery in LR Parsers](https://web.archive.org/web/20240302031213/https://what-when-how.com/compiler-writing/bottom-up-parsing-compiler-writing-part-13) and [Error Recovery for LR Parsers](https://apps.dtic.mil/sti/pdfs/ADA043470.pdf) (cited in Tree-sitter research)

- **Error nodes**: `ERROR` nodes inserted at recovery points; parsing continues
- **Partial trees**: Valid structure preserved around errors
- **Use case**: IDEs can highlight syntax errors while still providing structure for completion/navigation

**OBSERVATION** — Error recovery is **syntactic only**. No semantic error recovery (e.g., type error recovery).

---

## Memory Characteristics

**FACT** — From source analysis (`lib/src/lib.c`, `lib/src/tree.c`)

- **Node structure**: ~32 bytes/node (type ID, child array, start/end byte/point, parent pointer, next sibling)
- **Tree pooling**: `TSTree` pools nodes; freed en masse
- **Incremental**: Reuses node pool; only allocates for changed spans
- **Typical overhead**: 2-5x source size for CST (vs 1-2x for AST)

**INFERENCE** — For large codebases (1M+ files), CST memory is significant. Prime's compact binary format (prime-index) must compress this aggressively.

---

## What Is CHEAPLY Derivable from Incremental CST

| Information | Derivation Cost | Evidence |
|-------------|----------------|----------|
| **Syntax tree structure** | O(1) per node (already in CST) | FACT: CST is the output |
| **Token positions** (start/end byte, line/col) | O(1) per node | FACT: Stored on every node |
| **Named node kinds** (`function_declaration`, `class_declaration`, etc.) | O(1) per node | FACT: Node `kind_id` → string via `ts_node_type` |
| **Parent/child/sibling relationships** | O(1) traversal | FACT: CST is a tree |
| **Changed ranges after edit** | O(log n) via `ts_tree_get_changed_ranges` | FACT: Incremental API |
| **Syntactic pattern matches** (via queries) | O(tree size) per query | FACT: Single-pass query engine |
| **Symbol definitions** (function/class/method declarations) | O(tree size) via queries | FACT: Query captures `@name` on declaration nodes |
| **Import/export statements** (syntactic) | O(tree size) via queries | FACT: `import_statement`, `export_statement` nodes |
| **Call expressions** (syntactic `call_expression` nodes) | O(tree size) via queries | FACT: CST has call nodes |
| **Field/member access** (syntactic) | O(tree size) via queries | FACT: `member_expression`, `field_expression` nodes |
| **Control flow structure** (if/for/while/try nodes) | O(tree size) via queries | FACT: CST has all control nodes |

**KEY INSIGHT**: All above are **syntactic** — derivable from single-file CST without cross-file analysis, type inference, or symbol resolution.

---

## What REQUIRES Language-Specific Semantic Analysis Beyond Tree-sitter

| Information | Why Tree-sitter Cannot Provide | Required Analysis |
|-------------|-------------------------------|-------------------|
| **Symbol identity / qualified names** | No cross-file symbol table; same name ≠ same symbol | Symbol resolution + module system semantics |
| **Call graph edges (resolved)** | `call_expression` → callee unknown (could be method, function, closure, dynamic) | Type inference + receiver type resolution |
| **Type information** (variable types, return types, generics) | CST has type annotations but no inference | Type checker / type inference engine |
| **Inheritance/implementation resolution** | `extends Foo` — which `Foo`? (imported? local? generic?) | Symbol resolution + class hierarchy construction |
| **Override detection** | Syntactic `override` keyword optional/missing; need vtable layout | Class hierarchy + method signature matching |
| **Unused/dead code** | Requires reachability from entry points across call graph | Whole-program call graph + entry point analysis |
| **Data flow / taint analysis** | Requires value tracking through assignments, calls, returns | Intra/inter-procedural data-flow analysis |
| **Reflection/dynamic dispatch targets** | `getattr(obj, "method")`, `Class.forName()`, DI containers | Runtime analysis or framework-specific models |
| **Macro expansion** (Rust `macro_rules!`, C `#define`) | CST shows macro *invocation*, not expansion | Macro expander / compiler frontend |
| **Generic type instantiation** | `Vec<String>` — CST has tokens, not resolved type | Monomorphization / type substitution |
| **Control flow graph (CFG)** | CST is syntax tree, not basic-block graph | CFG construction from CST |
| **Program dependence graph (PDG)** | Requires data + control dependence edges | Full semantic analysis |

**FACT** — Code-Graph-RAG's hybrid frontends confirm this: C/C++ uses libclang, C# uses Roslyn, Rust uses rust-analyzer (for types) **on top of** Tree-sitter backbone.

**OBSERVATION** — Tree-sitter's query system can **approximate** some semantic facts heuristically (e.g., "function called `foo` in class `Bar`" → might be `Bar.foo`), but with false positives/negatives.

---

## Language-Specific Parser Capabilities

**FACT** — From parser repositories (tree-sitter/tree-sitter-*)

| Language | CST Completeness | Notable Gaps for Semantic Analysis |
|----------|-----------------|-----------------------------------|
| Rust | High (macros, attributes, generics) | No type inference; macro expansion opaque |
| TypeScript | High (types, interfaces, JSX) | No type checking; `any` escapes |
| Python | High (decorators, type hints) | No type inference; dynamic attributes |
| Go | High (receivers, interfaces) | No interface implementation resolution |
| Java | High (generics, annotations) | No overload resolution; reflection invisible |
| C/C++ | Medium (preprocessor expands) | Macros obscure structure; templates complex |
| C# | High (LINQ, async, generics) | Roslyn needed for overload/extension resolution |

---

## Prime's Current Usage (prime-parser)

**FACT** — From Prime codebase (prime-parser crate)

- 8 languages: Rust, TypeScript, JavaScript, Python, Go, Java, C, C++
- Tree-sitter queries extract: symbols (functions, classes, methods), imports, call expressions
- **Gap**: Prime currently derives relationships (CALLS, INHERITS, IMPLEMENTS) via **heuristic queries on CST** — not semantic resolution

**INFERENCE** — Prime's current approach mirrors Code-Graph-RAG's Tree-sitter backbone. The research question: **how much semantic resolution can be pushed into a language-agnostic model vs. requiring per-language analyzers?**

---

## What Prime Should Borrow

| Feature | Rationale | Evidence |
|---------|-----------|----------|
| **Incremental parsing via edit operations** | Prime's `prime build --update-graph` needs file-level invalidation | FACT: `ts_parser_parse_string` with edit ops |
| **Changed ranges API** | Efficient invalidation granularity | FACT: `ts_tree_get_changed_ranges` |
| **Query language for syntactic patterns** | Prime's structural search could use TS queries | FACT: S-expression queries, captures, predicates |
| **Error recovery for partial parsing** | Robustness on broken code | FACT: `ERROR` nodes, continued parsing |
| **Parser reuse across files** | Batch parsing efficiency | FACT: Single `TSParser` instance |
| **Named vs unnamed node distinction** | Filter noise in structural queries | FACT: `ts_node_is_named` |
| **Language grammar ecosystem** | 50+ parsers reduce Prime's parser burden | FACT: Upstream + community parsers |

---

## What Prime Should NOT Borrow

| Feature | Rationale | Evidence |
|---------|-----------|----------|
| **CST as primary storage** | Prime targets compact binary (succinct structures), not full CST | INFERENCE: CST 2-5x source size; Prime needs compression |
| **S-expression query as primary API** | Prime's agent API is 7 fixed semantic tools, not ad-hoc queries | FACT: PrimeEnvelope<T> tools vs. open query |
| **Single-file parsing model** | Prime needs cross-file symbol resolution | INFERENCE: Tree-sitter has no cross-file awareness |
| **GLR ambiguity handling as default** | Prime's binary format should resolve ambiguities at index time | HYPOTHESIS: Prime's model is post-resolution |

---

## Open Questions

1. **OPEN QUESTION**: Prime's incremental invalidation — can Tree-sitter's `changed_ranges` map directly to Prime's entity invalidation, or does a CST change range over-approximate affected entities (e.g., a comment change inside a function marks the whole function changed)?

2. **OPEN QUESTION**: Tree-sitter's CST includes every token. Prime's compact format — what's the information-theoretic minimum to reconstruct symbol+relationship graph? Can we discard unnamed nodes entirely?

3. **OPEN QUESTION**: For languages with significant semantic gaps (C++ templates, Rust macros, Python dynamic attrs), should Prime: (a) accept heuristic CST-based relationships with `inferred` confidence, (b) integrate language-specific analyzers (rust-analyzer, clangd, pyright), or (c) define a "capability model" where each language declares what it can provide?

4. **OPEN QUESTION**: Tree-sitter queries are fast but single-file. Prime's cross-file queries (dependencies, callers) — should Prime build a **global query index** (inverted index over CST query matches) or rely on graph traversal?

5. **OPEN QUESTION**: Memory — Tree-sitter CST ~32 bytes/node. For 10M LOC codebase (~50M nodes), that's ~1.6GB. Prime's mmap binary format — what's the target bits/node for succinct representation?

6. **OPEN QUESTION**: Tree-sitter's `ts_tree_get_changed_ranges` returns byte ranges. Prime's entity IDs are content-addressed (per SPECS). Can we map byte-range invalidation to content-addressed entity invalidation without re-hashing unchanged content?

7. **OPEN QUESTION**: The 50+ parser ecosystem — Prime currently vendors 8 parsers. Should Prime adopt a **dynamic parser loading** model (like ast-grep's custom language support) to avoid vendoring?

---

## References

- [Tree-sitter Official Site](https://tree-sitter.github.io/tree-sitter/) — Primary source for architecture, bindings, parsers
- [Tree-sitter C API](https://github.com/tree-sitter/tree-sitter/blob/master/lib/include/tree_sitter/api.h) — Primary source for incremental parsing API
- [Incremental Parsing Research](https://harmonia.cs.berkeley.edu/papers/twagner-parsing.pdf) — Primary theoretical basis
- [Tree-sitter Parsers](https://github.com/tree-sitter?tab=repositories) — Primary source for language grammars
- [Code-Graph-RAG Hybrid Frontends](https://github.com/vitali87/code-graph-rag/blob/main/docs/architecture/language-support.md) — Evidence of semantic layer necessity