Yes. I would give the research agent a **very strict prompt** so it understands that `Prime/` is a **research project**, not the Prime product itself.

You can paste this directly into your coding/research agent:

```text
# PRIME — CODEBASE KNOWLEDGE RESEARCH PROJECT

You are building a comprehensive technical research repository in the folder:

    ./Prime

IMPORTANT:

Prime is NOT the product implementation.

Prime is the RESEARCH that will be used to design and eventually build a new system called Prime.

Do not implement the Prime product.
Do not build a graph engine.
Do not build a binary format.
Do not write production code for the final system.

Your job is to investigate the problem deeply enough that another engineering team can later build Prime from the research with minimal unknowns.

---

# CORE PRODUCT IDEA

The future product, Prime, is intended to:

> Turn a codebase into a single, universal knowledge artifact that an agent can read, navigate, and reason about.

The artifact should eventually be:

- derived from the source code
- language agnostic at the representation level
- extremely compact
- extremely fast to retrieve from
- optimized for I/O
- optimized for codebase analysis
- optimized for agent knowledge retrieval
- capable of representing highly structured codebases
- deterministic wherever possible
- independent of the original source language
- represented as a single portable artifact
- usable without requiring an agent to understand the original repository structure

But NONE of these assumptions should automatically become implementation decisions.

Your job is to research the problem and determine:

1. What is technically possible?
2. What already exists?
3. What works?
4. What does not work?
5. Why?
6. What should be reused?
7. What should be avoided?
8. What remains unsolved?
9. What architecture would the evidence support?
10. What experiments are necessary before making decisions?

---

# FUNDAMENTAL SEPARATION

Keep these three things completely separate:

    RESEARCH
        ↓
    PRIME PRODUCT
        ↓
    ACC INTEGRATION

This repository is ONLY the first layer.

Do not mix the research repository with:

- Prime implementation
- Prime source code
- ACC implementation
- ACC configuration
- production APIs
- final binary format
- final graph implementation

ACC is an existing project:

    https://github.com/EnzoVezzaro/agents-code-context

It will eventually consume Prime.

Do not modify ACC.

Do not design ACC configuration yet.

Only research information that may eventually be relevant to integrating Prime into ACC.

---

# RESEARCH OBJECTIVE

Investigate the complete technical problem of representing a software repository as a compact, queryable, agent-oriented knowledge artifact.

The research should cover:

    SOURCE CODE
        ↓
    PARSING
        ↓
    SEMANTIC ANALYSIS
        ↓
    CODEBASE KNOWLEDGE
        ↓
    REPRESENTATION
        ↓
    STORAGE
        ↓
    COMPRESSION
        ↓
    INDEXING
        ↓
    RETRIEVAL
        ↓
    AGENT CONTEXT
        ↓
    AGENT REASONING

Do not assume that a graph is necessarily the correct solution.

Investigate graphs, indexes, relational representations, columnar representations, hybrid representations, and specialized structures.

The eventual product may be a graph internally, but the research must determine whether that is actually the best abstraction.

---

# RESEARCH PRINCIPLES

Follow these principles throughout the research.

## 1. Evidence over assumptions

Never write:

    "X is faster."

unless there is evidence.

Instead write:

    "Benchmark/source X reports..."
    "Our experiment indicates..."
    "Theoretical analysis suggests..."
    "This remains unverified."

Clearly distinguish:

    FACT
    OBSERVATION
    HYPOTHESIS
    INFERENCE
    OPEN QUESTION

---

## 2. Primary sources first

Prefer:

- official documentation
- academic papers
- technical specifications
- original repositories
- engineering papers
- benchmark results
- source code
- original authors' technical writing

Use secondary sources only when useful.

Whenever possible, inspect the actual implementation rather than relying solely on marketing/documentation.

---

## 3. Do not prematurely converge

Do not decide that Prime should use:

- SQLite
- RocksDB
- DuckDB
- Tree-sitter
- protobuf
- FlatBuffers
- Cap'n Proto
- mmap
- a custom binary format
- a graph database
- vectors
- embeddings

until the research establishes why.

Every major technology choice must have a documented rationale.

---

## 4. Research alternatives fairly

For every major design area, investigate multiple approaches.

Example:

    Storage:
        SQLite
        RocksDB
        LMDB
        DuckDB
        custom binary
        memory-mapped structures
        columnar storage
        immutable indexes

Do not research only the technology you expect to use.

---

# REQUIRED RESEARCH AREAS

Create comprehensive research covering at least the following.

---

# 01 — CODEBASE KNOWLEDGE

Research what can actually be derived from source code.

Investigate:

- files
- directories
- packages
- modules
- symbols
- declarations
- types
- functions
- methods
- classes
- interfaces
- variables
- constants
- parameters
- imports
- exports
- references
- calls
- inheritance
- implementations
- instantiation
- reads
- writes
- control flow
- data flow
- dependencies
- reverse dependencies
- tests
- configuration
- resources
- generated code
- build systems
- package managers
- architecture
- runtime relationships
- source provenance
- version information

Determine which information is:

    directly observable
    statically derivable
    inferable
    probabilistic
    unavailable without execution

Determine which information is useful to agents.

---

# 02 — CODE PROPERTY GRAPHS

Deeply research:

- Code Property Graph
- Joern
- CPG specifications
- AST graphs
- CFG
- PDG
- data-flow graphs
- call graphs
- dependency graphs
- semantic graphs

For each system document:

- purpose
- data model
- node model
- edge model
- storage model
- indexing
- query model
- language support
- scalability
- performance
- incremental updates
- strengths
- weaknesses
- limitations
- reusable components
- relevant source code

Study Joern deeply.

Official sources:

- https://github.com/joernio/joern
- https://docs.joern.io/
- https://cpg.joern.io/

Do not merely summarize marketing material.

---

# 03 — SCIP

Research SCIP in depth.

Repository:

https://github.com/sourcegraph/scip

Investigate:

- symbol identity
- symbol indexing
- definitions
- references
- implementations
- relationships
- cross-language representation
- serialization
- protobuf schema
- index generation
- storage
- incremental behavior
- limitations

Determine which concepts Prime could reuse.

---

# 04 — LSIF

Research LSIF deeply.

Repository:

https://github.com/microsoft/lsif-node

Investigate:

- why LSIF exists
- graph representation
- vertices
- edges
- source locations
- definitions
- references
- implementation
- persistence
- querying
- language-server integration
- limitations
- lessons for Prime

---

# 05 — TREE-SITTER

Research:

https://github.com/tree-sitter/tree-sitter

and:

https://tree-sitter.github.io/tree-sitter/

Investigate:

- parsing architecture
- concrete syntax trees
- incremental parsing
- error recovery
- language support
- grammar model
- performance
- memory usage
- parsing large repositories
- semantic limitations
- ecosystem
- bindings
- reusable components

Determine what Tree-sitter can provide and what Prime would need above it.

---

# 06 — AGENT-ORIENTED CODE INDEXING

Research current systems designed specifically for coding agents.

Include:

- Graph-sitter
- codebase-index
- Sourcegraph systems
- Cursor-style repository indexing
- Continue
- Aider
- OpenHands
- SWE-agent
- other significant open-source agent/codebase retrieval systems

For each determine:

- what information they index
- how they retrieve it
- whether they use ASTs
- whether they use graphs
- whether they use embeddings
- whether they use search
- how context is constructed
- what their bottlenecks are
- what information agents actually consume

Graph-sitter:

https://github.com/ast-grep/ast-grep

Research the current Graph-sitter implementation/documentation carefully rather than relying on assumptions.

---

# 07 — INFORMATION RETRIEVAL

Research:

- inverted indexes
- lexical search
- symbol search
- structural search
- semantic search
- vector search
- hybrid search
- graph search
- ranking
- filtering
- faceting
- query expansion
- retrieval precision
- retrieval recall

Determine which retrieval techniques are appropriate for deterministic codebase knowledge.

---

# 08 — STORAGE SYSTEMS

Deeply research:

- SQLite
- DuckDB
- RocksDB
- LevelDB
- LMDB
- sled
- redb
- Badger
- custom binary formats
- memory-mapped files
- columnar formats
- immutable files
- append-only formats
- B-trees
- LSM trees
- adjacency lists
- CSR/CSC structures

For each investigate:

- read performance
- random access
- sequential access
- memory usage
- file size
- write complexity
- update complexity
- concurrency
- portability
- mmap compatibility
- scalability

Do not recommend one yet.

---

# 09 — BINARY FORMAT DESIGN

Research:

- protobuf
- FlatBuffers
- Cap'n Proto
- MessagePack
- CBOR
- Apache Arrow
- custom binary layouts
- zero-copy formats
- memory-mappable formats

Investigate:

- serialization overhead
- deserialization overhead
- random access
- zero-copy access
- schema evolution
- compression
- file size
- implementation complexity

Determine whether a custom format could realistically outperform existing formats for this specific workload.

---

# 10 — COMPRESSION

Research deeply:

### Integer compression

- varints
- SIMD-BP128
- Stream VByte
- PForDelta
- Frame-of-Reference
- Elias coding

### Graph compression

- delta encoding
- adjacency compression
- WebGraph
- succinct graphs
- compressed sparse representations

### Strings

- dictionary encoding
- string interning
- front coding
- tries
- FSTs
- suffix structures

### General compression

- zstd
- lz4
- brotli
- gzip
- lzma

Determine the tradeoff between:

    compression ratio
    CPU cost
    random access
    decompression cost
    I/O reduction

The objective is NOT simply:

    smallest file

The actual research question is:

    smallest useful representation
    with fastest retrieval.

---

# 11 — SUCCINCT DATA STRUCTURES

Research:

- succinct trees
- succinct graphs
- bit vectors
- rank/select
- compressed bitmaps
- Roaring bitmaps
- Elias-Fano
- minimal perfect hashing
- FSTs
- wavelet trees

Determine whether these structures could materially improve Prime's storage efficiency.

---

# 12 — MEMORY MAPPING AND I/O

Research:

- mmap
- page cache
- page faults
- sequential vs random reads
- SSD behavior
- NVMe behavior
- filesystem caching
- read amplification
- memory locality
- CPU cache locality
- NUMA
- prefetching
- zero-copy access

Prime's performance should ultimately be measured in:

    bytes read
    pages touched
    allocations
    CPU cycles
    latency

not simply "query speed."

---

# 13 — LARGE-SCALE CODEBASES

Research how existing systems handle:

- 100K files
- 1M files
- millions of symbols
- tens of millions of relationships
- monorepos
- generated code
- vendored dependencies
- duplicated code
- multi-language repositories

Identify scaling bottlenecks.

---

# 14 — INCREMENTAL ANALYSIS

Research:

- incremental parsing
- incremental indexing
- dependency invalidation
- content hashing
- Merkle trees
- change detection
- partial recompilation
- persistent indexes
- immutable snapshots

Determine what must be recomputed when:

    one file changes
    one symbol changes
    one dependency changes
    an entire package changes

---

# 15 — LANGUAGE AGNOSTICISM

Research how different languages represent:

- modules
- namespaces
- packages
- classes
- traits
- interfaces
- generics
- macros
- decorators
- closures
- async functions
- pattern matching
- operator overloads
- type aliases
- dynamic dispatch

Investigate existing language-agnostic semantic models.

Determine what a universal representation can realistically preserve.

---

# 16 — AGENT CONTEXT AND TOKEN EFFICIENCY

Research how coding agents actually consume repository information.

Investigate:

- context windows
- tool calls
- progressive disclosure
- retrieval granularity
- context selection
- context compression
- token costs
- hallucination caused by incomplete context
- structural context
- source context
- provenance

Determine:

> What is the minimum information an agent needs to understand a code entity?

This is a central research question.

---

# 17 — REUSABLE OPEN-SOURCE TOOLS

For every relevant technology, identify:

- GitHub repository
- license
- language
- maturity
- maintenance status
- performance characteristics
- reusable libraries
- APIs
- format specifications
- potential integration value

Categorize each:

    REUSE
    ADAPT
    STUDY
    REPLACE
    AVOID

Never recommend a dependency without checking its actual repository and license.

---

# 18 — ACADEMIC RESEARCH

Search academic literature for:

- code graphs
- program graphs
- compressed graphs
- graph databases
- code indexing
- program analysis
- repository mining
- source-code retrieval
- code search
- code representation
- code embeddings
- agentic code retrieval
- compressed indexes
- succinct data structures
- large-scale static analysis

For important papers record:

- title
- authors
- year
- venue
- URL/DOI
- problem
- methodology
- results
- limitations
- relevance

---

# 19 — BENCHMARK RESEARCH

Find existing benchmarks for:

- parsing
- code search
- static analysis
- graph traversal
- indexing
- storage
- compression
- repository retrieval
- coding agents

Also design a future Prime benchmark methodology.

Do not run large experiments yet unless useful for validating a research question.

---

# REQUIRED OUTPUT STRUCTURE

Create the following structure:

Prime/

├── README.md
│
├── RESEARCH.md
│
├── findings/
│   ├── executive-summary.md
│   ├── key-findings.md
│   ├── technical-findings.md
│   ├── open-problems.md
│   └── research-gaps.md
│
├── prior-art/
│   ├── scip.md
│   ├── lsif.md
│   ├── cpg-joern.md
│   ├── tree-sitter.md
│   ├── graph-sitter.md
│   ├── agent-indexers.md
│   └── comparison.md
│
├── code-analysis/
│   ├── parsing.md
│   ├── ast-cst.md
│   ├── symbols.md
│   ├── type-analysis.md
│   ├── references.md
│   ├── call-graphs.md
│   ├── dependency-analysis.md
│   ├── data-flow.md
│   ├── control-flow.md
│   └── architecture-analysis.md
│
├── storage/
│   ├── databases.md
│   ├── binary-formats.md
│   ├── mmap.md
│   ├── columnar.md
│   └── custom-storage.md
│
├── compression/
│   ├── integer-compression.md
│   ├── graph-compression.md
│   ├── string-compression.md
│   └── general-compression.md
│
├── indexing/
│   ├── symbol-indexes.md
│   ├── graph-indexes.md
│   ├── search-indexes.md
│   └── succinct-structures.md
│
├── retrieval/
│   ├── information-retrieval.md
│   ├── agent-retrieval.md
│   ├── context-selection.md
│   └── token-efficiency.md
│
├── systems/
│   ├── io.md
│   ├── memory.md
│   ├── caching.md
│   ├── concurrency.md
│   └── scalability.md
│
├── languages/
│   ├── language-agnostic-models.md
│   ├── typescript.md
│   ├── rust.md
│   ├── python.md
│   ├── go.md
│   ├── java.md
│   └── other-languages.md
│
├── incremental/
│   ├── incremental-analysis.md
│   ├── invalidation.md
│   └── snapshots.md
│
├── reusable-tools/
│   ├── parsers.md
│   ├── analyzers.md
│   ├── storage.md
│   ├── compression.md
│   └── search.md
│
├── benchmarks/
│   ├── existing-benchmarks.md
│   ├── benchmark-methodology.md
│   └── datasets.md
│
└── references/
    ├── papers.md
    ├── repositories.md
    ├── specifications.md
    └── glossary.md

---

# README.md

The README should explain that this repository is a research project.

Use wording similar to:

"Prime is a research project investigating how software repositories can be transformed into extremely compact, language-agnostic knowledge representations optimized for machine analysis and AI-agent retrieval."

IMPORTANT:

Do NOT claim that Prime has already been implemented.

Do NOT describe an invented architecture as fact.

Clearly distinguish research from the future product.

---

# RESEARCH.md

This is the central research document.

It should contain:

1. Research objective
2. Problem definition
3. Scope
4. Research methodology
5. Existing approaches
6. Technical domains investigated
7. Key findings
8. Contradictions
9. Tradeoffs
10. Open problems
11. Research gaps
12. Preliminary conclusions

The document should link to deeper research files.

---

# COMPARISON TABLES

Create detailed comparison tables.

At minimum compare:

    SCIP
    LSIF
    CPG / Joern
    Tree-sitter
    Graph-sitter
    Sourcegraph-style indexing
    agent-oriented code indexes
    SQLite
    RocksDB
    LMDB
    DuckDB
    custom binary storage
    columnar storage

Compare:

- representation
- purpose
- storage
- indexing
- retrieval
- scalability
- language support
- incremental updates
- compression
- random access
- agent suitability
- weaknesses
- reusable components

---

# RESEARCH QUALITY

This must be serious technical research.

Do not produce generic AI summaries.

For every important technology:

1. Find the official repository.
2. Read the documentation.
3. Inspect relevant source code when possible.
4. Find papers/specifications.
5. Identify architectural decisions.
6. Identify limitations.
7. Identify benchmarks.
8. Record reusable components.
9. Record licensing.
10. Compare against alternatives.

Use direct URLs/references in the documents.

---

# NO PRODUCT DESIGN YET

This rule is extremely important.

Do NOT decide:

- Prime's final graph model
- Prime's final binary format
- Prime's final API
- Prime's final storage engine
- Prime's final compression algorithm
- Prime's final programming language
- Prime's final architecture

unless the research phase explicitly reaches that conclusion.

Instead document:

    evidence
    alternatives
    tradeoffs
    hypotheses
    unresolved questions

The eventual Prime product will be designed FROM this research.

---

# FINAL RESEARCH DELIVERABLE

At the end of the research phase, the folder must answer:

1. What can be extracted from a codebase?
2. What information matters to agents?
3. How do existing systems represent it?
4. What are their limitations?
5. What storage models are available?
6. What compression techniques are applicable?
7. What indexes are appropriate?
8. What retrieval patterns matter?
9. What does "agent-readable" actually mean?
10. What can be represented language-agnostically?
11. What scales to huge repositories?
12. What can be incrementally updated?
13. What open-source components can be reused?
14. What research gaps remain?
15. What experiments should be performed?
16. What conclusions can responsibly be drawn?

Only after these questions are answered should we begin designing the actual Prime product.

---

# WORKING METHOD

Work iteratively.

First:

1. Create the directory structure.
2. Create README.md.
3. Create RESEARCH.md.
4. Create a research index.
5. Begin with prior art.
6. Expand into the technical domains.
7. Maintain citations and source links.
8. Maintain a bibliography.
9. Maintain a glossary.
10. Maintain an explicit list of open questions.

Do not generate hundreds of shallow files.

Prefer fewer, deep, technically useful documents.

Whenever a research area becomes large, split it into focused documents.

---

# IMPORTANT: SEARCH THE WEB

This is a research task.

Use current web sources and GitHub repositories extensively.

Do not rely solely on your pretrained knowledge.

When researching a technology, prioritize:

1. official repository
2. official specification
3. academic paper
4. implementation/source
5. engineering articles
6. secondary explanations

Check current repository status, releases, licenses, and documentation.

---

# IMPORTANT: DO NOT IMPLEMENT PRIME

You are a research agent.

Your deliverable is the knowledge required to build Prime later.

The correct outcome of this task is not:

    "Prime is implemented."

The correct outcome is:

    "We now understand the technical landscape well enough to design Prime intelligently."

Start by creating `./Prime` and building the research repository.
```
---> 

Exactly. Then the research scope should be **technology-agnostic and deliberately extreme**.

The question is not:

> “What graph database should we use?”

It is:

> **“What combination of information theory, program analysis, agent architecture, compression, distributed systems, cryptography, indexing, and novel data structures gives an agent the maximum understanding of a codebase while requiring the minimum information transfer?”**

That opens Prime up to ideas that would normally seem unrelated.

## The Prime optimization target

I would formalize it as:

```text
              CODEBASE
                  │
                  ▼
          DERIVED KNOWLEDGE
                  │
                  ▼
        ┌───────────────────┐
        │       PRIME       │
        │                   │
        │ minimum useful    │
        │ information       │
        │                   │
        │ maximum agent     │
        │ answerability     │
        └─────────┬─────────┘
                  │
                  ▼
                AGENT
```

And optimize:

```text
               MAXIMIZE

    Questions answerable
    without source access

               DIVIDED BY

    bytes transferred
    + computation
    + latency
    + agent context
```

This is much closer to an **information-compression problem for machine intelligence** than a database problem.

---

# What we should investigate now

## 1. Information theory

This should be a first-class research area.

The key question:

> What information in source code is redundant from an agent's perspective?

We should investigate:

* entropy
* minimum description length
* sufficient statistics
* information bottleneck
* mutual information
* lossy semantic compression
* minimum sufficient representation
* rate–distortion theory
* information-preserving transformations

The interesting possibility is that Prime is effectively trying to discover a **rate–distortion function for software understanding**:

```text
less representation
        ↓
less information
        ↓
where is the threshold at which
agent capability meaningfully degrades?
```

That is much more fundamental than "compress the graph."

---

# 2. Knowledge representation

We should research whether the atomic unit should be:

```text
entity
fact
relationship
predicate
contract
behavior
invariant
capability
state transition
```

Potentially:

```text
subject → predicate → object
```

but with qualifiers:

```text
subject
predicate
object
scope
confidence
provenance
conditions
```

For example:

```text
AuthService.login
    RETURNS
Session
```

or:

```text
AuthService.login
    MAY_THROW
InvalidCredentials
    WHEN
password_verification_fails
```

The latter is more powerful because it represents **derived behavior**, not merely syntax.

---

# 3. Agent architecture

We need to study current and emerging agent architectures because the optimal Prime representation depends on the consumer.

Research:

```text
agent
├── model
├── context manager
├── memory
├── tool layer
├── retrieval
├── planning
├── execution
├── state
└── feedback
```

Questions:

* What does the model already remember?
* What gets cached?
* What is repeatedly retrieved?
* What does the model actually attend to?
* How are tools selected?
* How are tool results represented?
* Does the agent reason better over structured data than text?
* How much information can be supplied without distracting reasoning?
* Which information should be precomputed?
* What should remain query-time derived?

Prime should eventually optimize for the **actual information flow through the agent loop**, not just a hypothetical LLM.

---

# 4. Attention and transformer architecture

This is another area worth researching.

The representation might be optimized around how models consume context.

For example:

```text
100 KB of highly redundant text

versus

12 KB of structured, relationship-dense knowledge
```

The second could potentially be substantially more useful even if both contain "the same information."

We should research:

* attention complexity
* key/value caching
* context caching
* long-context degradation
* information density
* structured tool outputs
* model attention over JSON/trees/graphs/text
* context ordering
* retrieval chunk boundaries

The goal isn't to outsmart the model.

It's to understand **what representation makes the model's job easiest**.

---

# 5. Semantic hashing and content addressing

This is where crypto/P2P becomes interesting.

IPFS's content identifiers are hashes of content, and Merkle DAGs provide immutable, content-addressed structures where a root identifier commits to the entire descendant structure. ([Documentación IPFS][1])

That could be very useful for Prime even though Prime isn't a distributed storage system.

Imagine:

```text
Prime
  │
  ├── entity hash
  ├── relationship hash
  ├── module hash
  ├── package hash
  └── repository root hash
```

Then:

```text
repository state
      ↓
Prime root ID
```

This could give us:

* deterministic identity
* incremental invalidation
* deduplication
* trust
* reproducibility
* snapshot comparison
* distributed sharing

But we'd research this rather than assume we need Merkle DAGs.

---

# 6. P2P

P2P is interesting for a different reason.

Not:

> “Let's make Prime decentralized.”

But:

> **Can knowledge artifacts be distributed without centralizing the complete artifact?**

For example:

```text
              PRIME ROOT
                  │
        ┌─────────┼─────────┐
        ▼         ▼         ▼
      node A    node B    node C
       symbols   graph     context
```

A local agent could retrieve only the knowledge partition it needs.

Content addressing already provides a mechanism where content can be retrieved by identity rather than location. IPFS explicitly uses content identifiers and Merkle DAGs for this model. ([Documentación IPFS][1])

Potential research directions:

* content-addressed Prime shards
* distributed knowledge caches
* LAN sharing
* local team sharing
* cross-agent knowledge caches
* deduplicated organization-wide Prime artifacts
* peer-to-peer synchronization

This could become especially interesting for monorepos.

---

# 7. Cryptography

Crypto should absolutely be researched, but not because encryption makes retrieval faster.

Encryption generally makes direct retrieval harder.

The interesting crypto opportunities are:

### Integrity

A Prime artifact can cryptographically prove:

```text
"This knowledge corresponds to repository revision X."
```

### Provenance

Every fact could be associated with a commitment:

```text
fact
  ↓
source evidence
  ↓
hash / commitment
```

### Merkle proofs

An agent/tool could receive:

```text
"AuthService implements IUserService"
```

plus a compact proof that it belongs to the Prime artifact.

### Signed knowledge

Organizations could sign:

```text
Prime snapshot
```

so an agent can trust it without trusting the generating machine.

### Privacy-preserving retrieval

Searchable encryption is a real research area: encrypted indexes can support queries without simply decrypting everything, although they introduce security and performance tradeoffs. ([arXiv][2])

That's worth researching if Prime ever needs to operate over sensitive repositories.

### Zero-knowledge proofs

This is much more speculative, but worth investigating:

> Can we prove a derived property without exposing the source?

For example:

```text
"Does this service depend on database X?"
```

could theoretically become:

```text
YES + proof
```

without exposing implementation details.

Probably overkill for Prime v1.

But exactly the kind of idea the research should **not prematurely exclude**.

---

# 8. Succinct data structures

This area is directly relevant.

Succinct structures aim to represent information close to the theoretical lower bound while retaining fast operations. Rank/select structures are fundamental building blocks for compressed indexes, graphs and information retrieval. ([ScienceDirect][3])

Research:

* rank/select
* succinct bit vectors
* Elias-Fano
* wavelet trees
* compressed tries
* minimal perfect hashing
* succinct trees
* succinct graphs

Sux4J is an excellent open-source implementation/research reference for many of these structures. ([GitHub][4])

This could potentially let Prime represent millions of relationships in extremely little space while keeping direct lookup.

---

# 9. Compressed graph research

This is another place where Prime could steal ideas from outside software engineering.

WebGraph is explicitly designed to compress massive graphs while retaining useful graph access. ([GitHub][5])

Zuckerli extends this direction and specifically focuses on compressed graph storage with fast direct adjacency access without requiring complete decompression; its evaluation includes graphs up to billions of nodes and tens of billions of edges. ([Google Research][6])

That is directly relevant to your idea:

> **Small representation + direct retrieval.**

Prime's codebase graph is likely much more structured than a web graph, which might allow even more aggressive compression.

---

# 10. Approximate data structures

We should research:

* Bloom filters
* Cuckoo filters
* quotient filters
* learned indexes
* approximate membership
* sketching
* probabilistic indexes

A Bloom filter, for example, can answer set-membership queries with very little space but false positives. ([Docs.rs][7])

That creates interesting Prime possibilities:

```text
"Could this symbol exist here?"
        ↓
tiny probabilistic filter
        ↓
probably / definitely not
```

Then we only touch the expensive structure when necessary.

Roaring bitmaps are also worth investigating for compact sets of entity IDs and relationship memberships. ([Docs.rs][8])

---

# 11. Learned indexes

We should investigate a much less conventional idea:

> Can a tiny model replace parts of a traditional index?

For example:

```text
symbol hash
    ↓
tiny learned function
    ↓
approximate disk offset
```

Then:

```text
small model
+
small correction structure
```

could theoretically replace a much larger index.

This connects Prime with learned-index research rather than conventional database indexing.

---

# 12. Grammar compression

This is particularly promising for code.

Codebases are extraordinarily repetitive:

```text
patterns
identifiers
module structures
type relationships
AST patterns
API conventions
```

We should investigate:

* grammar-based compression
* Sequitur
* Re-Pair
* straight-line programs
* dictionary grammars
* DAG compression
* common subtree elimination

A codebase may have enough repeated structure that representing repeated semantic patterns once could be much more efficient than storing individual facts.

---

# 13. Knowledge deduplication

Another potentially huge opportunity:

```text
UserService
UserService
UserController
UserController
UserRepository
UserRepository
```

but also semantically repeated patterns:

```text
Service → Repository
Controller → Service
DTO → Validator
Entity → Repository
```

Prime might be able to represent:

```text
pattern 17
```

once and instantiate it across thousands of entities.

That's a form of **semantic compression**, not byte compression.

---

# 14. CRDTs and distributed updates

CRDTs probably aren't useful for the core artifact, but they're worth researching for Prime's **distributed/incremental side**.

CRDTs are designed so independent replicas can update state and deterministically converge, including in peer-to-peer environments. ([arXiv][9])

A future Prime ecosystem could potentially have:

```text
developer A
     │
     ▼
Prime delta A
     │
     ├──────────┐
     │          │
developer B   developer C
     │          │
     └────┬─────┘
          ▼
     merged knowledge
```

But I would classify CRDTs as:

**research for distributed Prime**, not necessarily core Prime.

---

# 15. The research space should therefore be much wider

I would change the Prime research map to:

```text
research/
│
├── agents/
│   ├── architectures/
│   ├── context/
│   ├── memory/
│   ├── tools/
│   ├── attention/
│   └── information-needs/
│
├── software/
│   ├── parsing/
│   ├── semantics/
│   ├── program-analysis/
│   └── architecture/
│
├── representation/
│   ├── knowledge-units/
│   ├── graphs/
│   ├── facts/
│   ├── semantics/
│   └── lossy-representation/
│
├── information-theory/
│   ├── entropy/
│   ├── information-bottleneck/
│   ├── sufficient-statistics/
│   └── rate-distortion/
│
├── compression/
│   ├── graph/
│   ├── grammar/
│   ├── integer/
│   ├── strings/
│   └── succinct/
│
├── indexing/
│   ├── traditional/
│   ├── inverted/
│   ├── learned/
│   ├── probabilistic/
│   └── approximate/
│
├── storage/
│   ├── mmap/
│   ├── binary/
│   ├── columnar/
│   └── cache/
│
├── distributed/
│   ├── p2p/
│   ├── content-addressing/
│   ├── merkle/
│   ├── crdt/
│   └── synchronization/
│
├── cryptography/
│   ├── commitments/
│   ├── provenance/
│   ├── proofs/
│   ├── searchable-encryption/
│   └── zero-knowledge/
│
├── retrieval/
│   ├── lexical/
│   ├── structural/
│   ├── semantic/
│   ├── graph/
│   └── agent-retrieval/
│
├── systems/
│   ├── io/
│   ├── memory/
│   ├── caching/
│   ├── simd/
│   └── hardware/
│
└── experiments/
```

That is the research repository I think you actually want.

---

# One particularly radical possibility

There is a possibility we should explicitly investigate:

### Prime may not need to be "one file" in the physical sense.

It could be **one logical artifact** whose knowledge units are content-addressed.

For example:

```text
                 PRIME ROOT
                     │
        ┌────────────┼────────────┐
        ▼            ▼            ▼
   structure     semantics     behavior
        │            │            │
        ▼            ▼            ▼
     content       content       content
      blocks        blocks        blocks
```

The agent experiences:

```text
Prime("repository")
```

as one artifact.

Underneath, it could be:

```text
local blocks
cache
P2P blocks
remote blocks
memory-mapped blocks
```

Content addressing/Merkle DAGs are already a proven way to give a collection of data a compact identity and permit verification of included content. ([Documentación IPFS][1])

But again, that's a **hypothesis worth testing**, not a decision.

---

# The research has a new north star

I would put this sentence into `init-prompt.md` and the eventual `README.md`:

> **Prime does not attempt to compress source code. Prime attempts to eliminate source code from the agent's information path by deriving the smallest possible knowledge representation that can answer the agent's questions directly.**

That is the real project.

And it means we should deliberately investigate **weird ideas**:

```text
information theory
       +
compiler theory
       +
graph compression
       +
probabilistic structures
       +
learned indexes
       +
cryptography
       +
content addressing
       +
P2P
       +
distributed systems
       +
agent architecture
       +
attention/context research
       +
systems programming
       ↓
     PRIME
```

The best Prime implementation may ultimately use only a few of those.

But **the research should not constrain itself to conventional code-indexing technology before we know what the optimal answer looks like.**

[1]: https://docs.ipfs.tech/concepts/content-addressing/?utm_source=chatgpt.com "Content Identifiers (CIDs) | IPFS Docs"
[2]: https://arxiv.org/abs/1709.09314?utm_source=chatgpt.com "An Efficiently Searchable Encrypted Data Structure for Range Queries"
[3]: https://www.sciencedirect.com/science/article/pii/S157106610900098X?utm_source=chatgpt.com "Rank and Select for Succinct Data Structures - ScienceDirect"
[4]: https://github.com/vigna/Sux4J?utm_source=chatgpt.com "GitHub - vigna/Sux4J: Sux4J is an effort to bring succinct data structures to Java. · GitHub"
[5]: https://github.com/vigna/webgraph?utm_source=chatgpt.com "GitHub - vigna/webgraph: WebGraph is a framework for graph compression. · GitHub"
[6]: https://research.google/pubs/zuckerli-a-new-compressed-representation-for-graphs/?utm_source=chatgpt.com "Zuckerli: A New Compressed Representation for Graphs"
[7]: https://docs.rs/bloomfilter/latest/bloomfilter/index.html?utm_source=chatgpt.com "bloomfilter - Rust"
[8]: https://docs.rs/roaring/latest/roaring/?utm_source=chatgpt.com "roaring - Rust"
[9]: https://arxiv.org/abs/1805.06358?utm_source=chatgpt.com "Conflict-free Replicated Data Types (CRDTs)"


----> 

Yes. That should be a **hard Prime requirement**, not merely a feature.

The target becomes:

> **Prime derives a universal knowledge representation from any codebase, regardless of language, repository size, or architecture, while preserving the smallest set of language-neutral facts required for agent understanding.**

That changes the research substantially.

## Language agnosticism needs two layers

We should not force languages into an artificial universal AST.

Instead:

```text
                    CODEBASE
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
       TypeScript     Rust        Python
          │            │            │
          ▼            ▼            ▼
      language-specific analysis
          │            │            │
          └────────────┼────────────┘
                       ▼
              UNIVERSAL SEMANTICS
                       │
                       ▼
                    PRIME
```

The language frontend can be completely different for each language.

Prime only requires that each frontend produce a common **semantic vocabulary**.

For example:

```text
Rust trait
TypeScript interface
Java interface
C++ abstract class
Python protocol / abstract base class
```

may all produce some form of:

```text
CONTRACT
```

while retaining language-specific attributes separately.

Likewise:

```text
fn
function
method
lambda
closure
```

can map into a universal callable model.

---

# But "any language" has to be defined carefully

There are really three levels.

### Level 1 — Parseable

Prime can understand the structure of the language.

```text
AST
symbols
files
modules
declarations
```

### Level 2 — Semantically analyzable

Prime can resolve:

```text
references
calls
types
imports
dependencies
implementations
```

### Level 3 — Knowledge derivable

Prime can derive:

```text
responsibilities
architecture
behavioral facts
impact
contracts
test relationships
```

Some languages will support Level 3 deeply.

Others won't.

Prime should **degrade gracefully** rather than pretend all languages have identical semantic information.

```text
                     PRIME
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
     exact          inferred       unavailable
     knowledge      knowledge      knowledge
```

And provenance/confidence needs to travel with the knowledge.

---

# Size independence is equally important

Prime should not be designed around "normal repositories."

We need:

```text
5 files
      ↓
500 files
      ↓
50,000 files
      ↓
500,000 files
      ↓
millions of files
```

The architecture should scale along at least three dimensions:

```text
files
entities
relationships
```

and ideally allow:

```text
incremental derivation
incremental invalidation
parallel analysis
partial loading
partial retrieval
```

A 50-file project should not require a heavyweight database.

A 5-million-entity monorepo should not require loading the entire Prime artifact into memory.

---

# Multi-language repositories are the real test

The strongest requirement isn't:

> "Prime supports 50 languages."

It's:

> **"Prime can understand a repository containing many languages simultaneously."**

For example:

```text
frontend/
  TypeScript

backend/
  Rust

services/
  Go

native/
  C++

scripts/
  Python

infra/
  Terraform
  YAML
```

The universal representation should preserve relationships across boundaries:

```text
TypeScript
   │
   ▼
HTTP API
   │
   ▼
Rust service
   │
   ▼
gRPC
   │
   ▼
Go service
   │
   ▼
database
```

That is much more valuable than language-specific parsing in isolation.

---

# Research track: language-neutral semantics

We should add a dedicated research area:

```text
research/
└── languages/
    ├── universal-semantics.md
    ├── language-frontends.md
    ├── cross-language-analysis.md
    ├── semantic-normalization.md
    ├── language-capabilities.md
    └── unsupported-semantics.md
```

And research:

* Tree-sitter grammars
* language servers / LSP
* SCIP language coverage
* compiler APIs
* type systems
* symbol resolution
* cross-language interfaces
* foreign-function boundaries
* generated code
* dynamic languages
* macro systems
* metaprogramming
* reflection
* runtime dispatch

The question is:

> **What is genuinely universal across programming languages?**

---

# We should define a capability model

Instead of:

```text
SUPPORTED: Python
NOT SUPPORTED: Haskell
```

Prime should think:

```text
Language
  ├── parsing: exact
  ├── symbols: exact
  ├── references: exact
  ├── types: partial
  ├── calls: partial
  ├── architecture: inferred
  └── runtime behavior: unavailable
```

So a language adapter can advertise capabilities.

Conceptually:

```text
language-capabilities:
  parsing
  declarations
  symbols
  references
  types
  calls
  data-flow
  control-flow
  modules
  packages
  macros
  runtime-semantics
```

This makes Prime extensible without weakening the universal model.

---

# Dynamic languages are especially important

Python, JavaScript, Ruby and similar languages demonstrate why we cannot equate:

```text
static syntax
=
complete semantics
```

Prime needs to distinguish:

```text
OBSERVED
DERIVED
INFERRED
UNKNOWN
```

For example:

```text
AuthService
calls
UserRepository.find
confidence: high
evidence: static reference
```

versus:

```text
AuthService
may call
PluginRegistry.resolve(...)
confidence: inferred
reason: dynamic dispatch
```

This becomes a core part of the knowledge model.

---

# Generated and unconventional code also matter

"Any codebase" means we need to investigate:

* generated source
* vendored dependencies
* minified code
* code with macros
* monorepos
* polyglot repositories
* schema-generated clients
* protobuf/OpenAPI generated code
* code embedded in templates
* configuration languages
* build files
* infrastructure-as-code

Prime doesn't necessarily need to treat all of these as ordinary source code.

It needs to understand their **role in the codebase**.

---

# The research goal becomes stronger

We now have four simultaneous constraints:

```text
                    PRIME
                      │
      ┌───────────────┼────────────────┐
      ▼               ▼                ▼
  LANGUAGE        SCALE             AGENT
  AGNOSTIC        AGNOSTIC          OPTIMIZED
      │               │                │
      └───────────────┼────────────────┘
                      ▼
               MINIMUM KNOWLEDGE
```

And one more:

```text
               DETERMINISTIC
                    when
                  possible
```

So the eventual Prime objective becomes:

> **Derive the smallest practical universal representation of a codebase, across languages and scales, that allows a next-generation agent to answer as many codebase questions as possible without retrieving the underlying source.**

That should now be one of the **top-level research requirements**.

And importantly, we should test the architecture against **small, large, and polyglot repositories from the beginning**, rather than proving it on a TypeScript project and adding language support later.
