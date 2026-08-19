Prime Research Project - Guidelines

This document outlines the guidelines for working with this research repository.

## Core Directives (from init-promt.md)

### 1. Evidence Over Assumptions
- Never write: "X is faster." unless there is evidence.
- Instead write: "Benchmark/source X reports...", "Our experiment indicates...", "Theoretical analysis suggests...", "This remains unverified."
- Clearly distinguish: FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION

### 2. Primary Sources First
- Prefer: official documentation, academic papers, technical specifications, original repositories, engineering papers, benchmark results, source code, original authors' technical writing
- Use secondary sources only when useful
- Whenever possible, inspect the actual implementation rather than relying solely on marketing/documentation

### 3. Do Not Prematurely Converge
- Do NOT decide that Prime should use: SQLite, RocksDB, DuckDB, Tree-sitter, protobuf, FlatBuffers, Cap'n Proto, mmap, a custom binary format, a graph database, vectors, embeddings
- Until the research establishes why, every major technology choice must have a documented rationale
- For every major design area, investigate multiple approaches (example given in storage section: SQLite, RocksDB, LMDB, DuckDB, custom binary, memory-mapped structures, columnar storage, immutable indexes)

### 2. Research Conduct
- **Do not generate hundreds of shallow files** - Prefer fewer, deep, technically useful documents
- **Whenever a research area becomes large, split it into focused documents** (this is why SPECS has many focused markdown files)
- **Do not produce generic AI summaries** - This must be serious technical research
- **For every important technology**, must:
  1. Find the official repository
  2. Read the documentation
  3. Inspect relevant source code when possible
  4. Find papers/specifications
  5. Identify architectural decisions
  6. Identify limitations
  7. Identify benchmarks
  8. Record reusable components
  9. Record licensing
  10. Compare against alternatives

- Use direct URLs/references in the documents

### 3. Technology Neutrality
- Do NOT force languages into an artificial universal AST
- Instead: each frontend produces a common semantic vocabulary (e.g., Rust trait, TypeScript interface, Java interface, C++ abstract class, Python protocol all produce some form of CONTRACT)
- Language adapters can advertise capabilities (parsing: exact/symbols: exact/references: exact/types: partial/calls: partial/architecture: inferred/runtime behavior: unavailable)

### 4. Knowledge Representation
- The atomic unit should be carefully considered (entity, fact, relationship, predicate, contract, behavior, invariant, state transition)
- Potential format: subject → predicate → object with qualifiers (subject, predicate, object, scope, confidence, provenance, conditions)
- Example: AuthService.login RETURNS Session or AuthService.login MAY_THROW InvalidCredentials WHEN password_verification_fails

### 5. Language Agnosticism
- Two layers: language-specific analysis → universal semantics → PRIME
- Do not force all languages into one AST; each language has its own frontend
- Universal vocabulary should be language-agnostic (e.g., CONTRACT for interfaces, CALLABLE for functions)
- Distinguish capability levels per language (Level 1: Parseable, Level 2: Semantically analyzable, Level 3: Knowledge derivable)
- Graceful degradation rather than pretending all languages have identical semantic information

### 6. Token Efficiency
- Central research question: "What is the minimum information an agent needs to understand a code entity?"
- Design for minimum useful representation (not smallest file, but smallest useful representation with fastest retrieval)
- Balance compression ratio against CPU cost, random access, decompression cost, I/O reduction
- Key insight: 12 KB of structured, relationship-dense knowledge can be substantially more useful than 100 KB of highly redundant text

### 7. Incremental Analysis
- Research incremental parsing, incremental indexing, dependency invalidation, content hashing, Merkle trees, change detection
- Determine what must be recomputed when: one file changes, one symbol changes, one dependency changes, an entire package changes
- Support partial loading and partial retrieval (5-million-entity monorepo should not require loading entire artifact into memory)

### 8. No Product Design Yet
- Do NOT decide: Prime's final graph model, binary format, API, storage engine, compression algorithm, programming language, architecture
- Unless research phase explicitly reaches that conclusion
- Instead document: evidence, alternatives, tradeoffs, hypotheses, unresolved questions
- The eventual Prime product will be designed FROM this research

### 9. Working Method
- Work iteratively:
  1. Create the directory structure
  2. Create README.md
  3. Create RESEARCH.md
  4. Create a research index
  5. Begin with prior art
  6. Expand into the technical domains
  7. Maintain citations and source links
  8. Maintain a bibliography
  9. Maintain a glossary
  10. Maintain an explicit list of open questions
- Do not generate hundreds of shallow files
- Prefer fewer, deep, technically useful documents
- Whenever a research area becomes large, split it into focused documents

## Document Standards
- All documents must be evidence-based, not AI-generated summaries
- Must distinguish between FACT, OBSERVATION, HYPOTHESIS, INFERENCE, OPEN QUESTION
- Must link to relevant sources (repositories, papers, specifications)
- Must maintain consistency with init-promt.md principles
- Must clearly separate research from product design

## Directory Conventions
- SPECS/ - The main research specification folder (already populated with 68 markdown files)
- docs/ - This folder, containing operational guidelines and getting-started documentation
- All markdown files should follow the style established in SPECS/ (no comments unless asked, concise, direct)
- Code references should use file_path:line_number pattern

## Quality Gates
Before marking research as complete for any area:
- [ ] Evidence cited for key claims
- [ ] Primary sources inspected (repositories, papers, specs)
- [ ] Alternatives considered and documented
- [ ] Tradeoffs recorded
- [ ] Limitations identified
- [ ] Reusable components recorded with licenses
- [ ] Open questions explicitly listed
- [ ] No product design claims without research consensus