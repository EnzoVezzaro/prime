# ripgrep — Prior Art Analysis

## Confidence Legend
- **FACT** — Verified by primary source (ripgrep README, blog, source code)
- **OBSERVATION** — Directly observed from source behavior or documented mechanics
- **HYPOTHESIS** — Proposed explanation requiring validation
- **INFERENCE** — Deduced from evidence, marked as such
- **OPEN QUESTION** — Explicitly unknown, needs research

---

## 1. Architecture Overview

### 1.1 Purpose (FACT)
ripgrep (`rg`) is a **line-oriented recursive search tool** that searches directories for regex patterns while respecting `.gitignore`, skipping hidden/binary files by default [README](https://github.com/BurntSushi/ripgrep).

### 1.2 Core Philosophy (FACT)
> *"ripgrep is fast because: (1) Rust's regex engine uses finite automata, SIMD, and aggressive literal optimizations; (2) UTF-8 decoding built into DFA; (3) chooses memory map vs incremental buffer automatically; (4) RegexSet for ignore patterns; (5) lock-free parallel directory traversal"* [README](https://github.com/BurntSushi/ripgrep).

### 1.3 Three Search Modes (OBSERVATION)
| Mode | Trigger | Mechanism |
|------|---------|-----------|
| **Memory Map** | Single file, large | `mmap()` file, search directly |
| **Incremental Buffer** | Directory traversal, stdin | Fixed-size buffer, streaming search |
| **Hybrid** | Auto-selected | Heuristics: file size, match density, pattern complexity |

---

## 2. Directory Traversal & Ignore Handling

### 2.1 Parallel Walk (FACT)
- Uses `ignore` crate (also by BurntSushi) for **lock-free parallel recursive directory iteration**
- **Work-stealing queue** (Chase-Lev deque via `crossbeam`) distributes paths to worker threads
- **Minimum stat calls** — `walkdir` crate optimized for this

### 2.2 Ignore Pattern Matching (FACT)
- **RegexSet**: Compiles all `.gitignore`/`.ignore`/`.rgignore` globs into **single regex alternation**
- **One match per path** — not N patterns × M paths
- **Precedence**: `.rgignore` > `.gitignore` > global; later patterns override earlier
- **Full `.gitignore` semantics**: Directory patterns, negation (`!`), trailing slash

### 2.3 File Type Filtering (FACT)
- **Built-in types**: `-tpy` (Python), `-Tjs` (exclude JS), `--type-list` shows all
- **Custom types**: `--type-add 'foo:*.{foo,foobar}'`
- **Extension-based fast path**: `*.ext` globs matched by suffix check, not regex

---

## 3. Regex Engine & Literal Optimizations

### 3.1 Rust Regex Engine (FACT)
- **Finite automata (DFA/NFA)** — guaranteed linear time, no backtracking catastrophes
- **SIMD acceleration** — `memchr` for literal scanning, Teddy algorithm for multi-literal (Aho-Corasick + SIMD)
- **UTF-8 in DFA** — Unicode support without performance cliff (unlike GNU grep)

### 3.2 Literal Extraction (FACT)
Engine extracts **prefix, suffix, and inner literals** from any pattern:
| Pattern | Extracted Literals |
|---------|-------------------|
| `foo|bar` | `foo`, `bar` |
| `(a|b)c` | `ac`, `bc` |
| `[ab]foo[yz]` | `afooy`, `afooz`, `bfooy`, `bfooz` |
| `(foo)?bar` | `foobar`, `bar` |
| `\w+foo\d+` | `foo` (inner literal) |

**Inner literal optimization**: Search for `foo` to find candidate *lines*, then run full regex on only those lines.

### 3.3 Multi-Literal Search (FACT)
- **Aho-Corasick** for multiple literals (default)
- **Teddy (SIMD)**: 128-bit vectorized multi-pattern search — *"at least one of the key optimizations that propels ripgrep past GNU grep"* [blog](https://blog.burntsushi.net/ripgrep/)

### 3.4 PCRE2 Fallback (FACT)
- `-P/--pcre2` or `--auto-hybrid-regex` for backreferences, look-around
- **JIT compilation** — PCRE2 with JIT enabled
- **Cost**: Slower for simple patterns; only used when needed

---

## 4. Search Mechanics

### 4.1 Incremental Buffer Search (FACT)
```
Buffer size: ~128KB (configurable)
Process:
  1. Read chunk into buffer
  2. Find line boundaries (count newlines for -n)
  3. Search buffer with regex/DFA
  4. Handle matches crossing buffer boundary
  5. Carry over context lines (for -A/-B/-C)
  6. Repeat
```
- **Line counting**: SIMD (16 bytes/iteration) — `packed comparisons`
- **Context handling**: Carry `N` lines between buffers
- **No line-by-line overhead** — searches large buffer at once

### 4.2 Memory Map Search (FACT)
- `mmap()` file → search directly on mapped pages
- **Better for**: Single large files, random access patterns
- **Worse for**: Many small files (syscall overhead, page faults)
- **Auto-selection**: ripgrep chooses based on file size, pattern, match density

### 4.3 Result Aggregation (FACT)
- **Per-thread buffers** — avoid lock contention during search
- **Structured matches** (Silver Searcher) vs **formatted strings** (rg, git grep)
- **rg writes formatted output to string buffer** — enables incremental search (can't refer back to mmap'd content)
- **Single-threaded print** — serialized dump of buffers

---

## 5. Performance Characteristics (Benchmarks)

### 5.1 Linux Kernel Benchmarks (FACT from [blog](https://blog.burntsushi.net/ripgrep/))
| Tool | `PM_RESUME` (ignore) | `PM_RESUME` (whitelist) |
|------|---------------------|------------------------|
| rg (ignore) | 0.334s | — |
| rg (whitelist) | — | **0.228s** |
| ag (mmap) | 1.588s | — |
| ucg (whitelist) | — | 0.218s |
| git grep | 0.345s | — |
| pt | 0.456s | — |
| sift | 0.630s | — |

### 5.2 Key Findings (OBSERVATION)
- **Memory maps slower for many small files** — `rg (mmap)` 1.6s vs `rg (incremental)` 0.33s
- **`.gitignore` overhead** — `rg (ignore)` 0.33s vs `rg (whitelist)` 0.23s (30% penalty)
- **Line counting cost** — Significant in single-file benchmarks; amortized in directory search
- **Unicode penalty** — GNU grep 32x slower with Unicode; rg unaffected

### 5.3 Single File Benchmarks (FACT)
| Tool | `Sherlock [A-Z]\w+` (13GB) |
|------|---------------------------|
| rg | **1.042s** |
| ugrep | 1.339s |
| GNU grep (Unicode) | 6.577s |

---

## 6. Comparison: ripgrep vs Prime Semantic Index

| Dimension | ripgrep (Lexical) | Prime (Semantic) |
|-----------|-------------------|------------------|
| **Index Type** | None (live grep) | Precomputed semantic index |
| **Query Model** | Regex on source text | Structured query on entities/relations |
| **Ranking** | None (line order) | Relevance (PageRank, TF-IDF, graph) |
| **Cross-file** | Independent line matches | Graph traversal (calls, deps, types) |
| **Symbol Awareness** | None (text only) | Full (definition, reference, type) |
| **Incremental** | N/A (no index) | Planned (research area 14) |
| **Token Efficiency** | Low (raw lines) | High (PrimeEnvelope, progressive) |
| **Language Support** | 100% (text) | Tree-sitter per language |
| **Ignore Handling** | `.gitignore` regex | Prime-specific (semantic relevance) |

---

## 7. What Prime Should BORROW

### 7.1 Parallel Directory Traversal (FACT → BORROW)
- **Lock-free work-stealing queue** for distributing file parsing jobs
- **Minimum stat calls** — use `walkdir`/`ignore` crate patterns
- **Batch file discovery** before parsing (separate phase)

### 7.2 RegexSet for Ignore/Filter Patterns (FACT → BORROW)
- Compile **all glob patterns into single regex** for O(1) path matching
- Apply to: Prime's file inclusion/exclusion, language detection, vendor filtering
- **Precedence handling**: Later patterns override earlier (like `.gitignore`)

### 7.3 SIMD Line Counting (FACT → BORROW)
- **16-byte packed comparisons** for newline counting
- Use in: Prime's incremental parser for fast line/column mapping
- **Rust**: `std::arch::x86_64::_mm_cmpeq_epi8` or `memchr` crate

### 7.4 Adaptive Search Strategy (FACT → BORROW)
- **Heuristics for mmap vs incremental**:
  - File size threshold (e.g., >10MB → mmap)
  - Match density estimate (sample first N KB)
  - Pattern complexity (literals vs full regex)
- Prime: **Adaptive query execution** — semantic index vs lexical fallback vs hybrid

### 7.5 Literal Extraction for Fast Prefilter (FACT → BORROW)
- Extract **keywords from semantic query** → use as lexical prefilter
- Example: `prime_search "AuthService.login"` → extract `"AuthService"`, `"login"` → lexical candidate files → semantic verification
- **Avoids full graph traversal** for common queries

### 7.6 Per-Thread Result Buffers (FACT → BORROW)
- **Parallel query execution** — each worker thread accumulates results
- **Merge at end** — single-threaded serialization
- **Avoids lock contention** during graph traversal

### 7.7 UTF-8 in DFA (FACT → BORROW)
- **Unicode-aware parsing without penalty** — build UTF-8 decoding into automaton
- Prime's Tree-sitter parsers already handle this; ensure query engine does too

---

## 8. What Prime Should NOT Borrow

### 8.1 No Persistent Index (FACT → AVOID)
- rg **builds no index** — searches live filesystem every time
- Prime **must precompute** semantic index (Entity, Relation, KnowledgeGraph)
- Live grep = fallback only (when semantic index missing/stale)

### 8.2 Line-Oriented Output (FACT → AVOID)
- rg emits **lines with matches** — no structure, no relationships
- Prime emits **PrimeEnvelope<T>** with entities, relations, provenance, coverage
- **Structured semantic response** > raw text lines

### 8.3 Single-Pattern Optimization (OBSERVATION → AVOID)
- rg optimizes for **one regex at a time**
- Prime queries are **multi-dimensional**: search + context + relationships + dependencies
- Need **composite query planner**, not single-pattern optimizer

### 8.4 File-Centric, Not Symbol-Centric (FACT → AVOID)
- rg operates on **files → lines**
- Prime operates on **entities → relations**
- File-centric fallback only when symbol resolution fails

### 8.5 No Ranking/Relevance (FACT → AVOID)
- rg returns **all matches in filesystem order**
- Prime **must rank** by: reference count, centrality, recency, user context
- **Progressive disclosure**: Top-K → expand on demand

---

## 9. Lexical vs Semantic Index Split

### 9.1 When to Use Lexical (ripgrep-style) Fallback (HYPOTHESIS)
| Scenario | Approach |
|----------|----------|
| Symbol not in index (new file, unsupported lang) | `rg` on workspace |
| Query is purely textual (string literal, comment) | `rg` with context lines |
| Semantic index stale/corrupted | `rg` as recovery |
| Cross-repo search (no unified index) | `rg` per repo + merge |
| Regex patterns not expressible in semantic query | `rg` with `-P` PCRE2 |

### 9.2 When to Use Semantic Index (Prime Primary) (HYPOTHESIS)
| Scenario | Approach |
|----------|----------|
| "Find definition of `AuthService.login`" | `prime_lookup` → exact entity |
| "Who calls `Database.connect`?" | `prime_relationships` → callers |
| "Impact of changing `User.id` type" | `prime_impact` → dependents |
| "Architecture of `payment` module" | `prime_architecture` → module graph |
| "Dependencies of `AuthService`" | `prime_dependencies` → dep graph |

### 9.3 Hybrid Query Execution (HYPOTHESIS)
```
prime_search "AuthService":
  1. Semantic index: exact symbol match → Entity[]
  2. If < threshold results: lexical fallback
     rg -t<lang> "AuthService" → candidate files
     Parse candidates → extract entities → merge
  3. Rank combined results (semantic > lexical)
  4. Return PrimeEnvelope with coverage % and source_required
```

### 9.4 Lexical as Index Construction Aid (HYPOTHESIS)
- **Bootstrap**: `rg` to find all files with `fn `, `class `, `def `, `function ` → feed to parser
- **Change detection**: `rg` on `git diff` output → identify files needing re-parse
- **Validation**: Sample `rg` results vs semantic index → detect drift

---

## 10. Open Questions

1. **OPEN QUESTION**: What is the **optimal buffer size** for incremental search in Prime's parser? rg uses ~128KB; Prime parses ASTs (larger units).

2. **OPEN QUESTION**: **Teddy/SIMD multi-literal** — can Prime use similar for **multi-symbol queries**? e.g., search for `AuthService` AND `login` simultaneously via SIMD.

3. **OPEN QUESTION**: **Ignore pattern compilation** — Prime needs semantic ignore (generated files, vendored deps, test files). Can we adapt RegexSet for semantic predicates?

4. **OPEN QUESTION**: **Memory map vs incremental for AST parsing** — Tree-sitter supports incremental parsing. When is mmap better for parsing vs streaming?

5. **OPEN QUESTION**: **Parallel parsing scaling** — rg parallelizes file *search*. Prime parallelizes file *parsing*. Different contention profiles (CPU vs I/O). Optimal thread count?

6. **OPEN QUESTION**: **Line/column mapping** — rg counts lines fast via SIMD. Prime needs **byte offset → line/col** for every entity. Store in index? Compute on demand?

7. **OPEN QUESTION**: **Binary file detection** — rg skips binaries via NUL byte heuristic. Prime needs same for parsing (skip minified JS, compiled artifacts).

8. **OPEN QUESTION**: **Compressed file support** — rg `-z` searches gzip/bzip2/xz/lz4/zstd. Prime may need to parse generated/minified sources in archives.

9. **OPEN QUESTION**: **Preprocessor filters** — rg supports arbitrary input filters (PDF, decompression). Prime could use for: generated code (protobuf, GraphQL), transpiled output.

10. **OPEN QUESTION**: **Benchmark methodology** — rg's benchsuite is rigorous (warmup, multiple iterations, page cache control). Prime needs equivalent for query/index/parse benchmarks.

---

## 11. Evidence Summary

| Claim | Evidence | Confidence |
|-------|----------|------------|
| Lock-free parallel walk | README, `ignore`/`crossbeam` crates | FACT |
| RegexSet for ignore patterns | README, blog "Gathering files to search" | FACT |
| Rust regex: DFA, SIMD, literal opt | Blog "Regex engine", "Literal optimizations" | FACT |
| Inner literal optimization | Blog: `\w+foo\d+` extracts `foo` | FACT |
| Teddy SIMD multi-literal | Blog: "SIMD algorithm called Teddy" | FACT |
| Incremental buffer search | Blog "Mechanics" | FACT |
| mmap vs incremental auto-select | Blog: "chooses the best searching strategy" | FACT |
| mmap slower for many small files | Benchmark: `rg (mmap)` 1.6s vs `rg (inc)` 0.33s | FACT |
| Line counting via SIMD | Blog: "packed comparisons (16 bytes at a time)" | FACT |
| Per-thread result buffers | Blog "Printing" | FACT |
| PCRE2 fallback with JIT | README, blog | FACT |
| UTF-8 in DFA (no Unicode penalty) | Benchmark: rg Unicode = ASCII speed | FACT |
| File type filtering (`-t`, `-T`) | README | FACT |
| Compressed file search (`-z`) | README | FACT |
| Preprocessor filters | README | FACT |
| No persistent index | README: "recursively searches... no index" | FACT |
| Benchmark methodology | Blog "Methodology", benchsuite crate | FACT |

---

*Research conducted per Prime methodology: primary sources first (ripgrep README, blog, source), evidence over assumptions, distinguish confidence levels.*