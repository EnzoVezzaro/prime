# Prime

<div align="center">

# Prime

### The smallest useful representation of a codebase for agents.

[![Status: Research](https://img.shields.io/badge/status-research-red?style=for-the-badge)](#status)
[![License: MIT](https://img.shields.io/badge/license-MIT-yellow.svg?style=for-the-badge)](LICENSE)

**Research into a universal, language-agnostic representation that lets next-generation agents understand and operate on codebases without retrieving the underlying source.**

</div>

---

> **What is the minimum information an agent needs to understand a codebase without reading its code?**

That is the question behind Prime.

Prime is a research project exploring how an entire software repository can be transformed into a **single, extremely compact knowledge artifact** containing the smallest useful units of information required for an agent to understand, navigate, analyze, and reason about the repository.

Prime does **not** aim to compress source code.

Prime aims to make the source code unnecessary for as many agent questions as possible.

```text
                         CODEBASE
                            │
                            ▼
                    ┌─────────────────┐
                    │     PRIME       │
                    │                 │
                    │ derived         │
                    │ knowledge       │
                    │                 │
                    │ minimal units   │
                    │ compact         │
                    │ indexed         │
                    │ language        │
                    │ agnostic        │
                    └────────┬────────┘
                             │
                             ▼
                           AGENT
                             │
                    ┌────────┴────────┐
                    ▼                 ▼
                 ANSWER             ACT
```

---

# The core idea

A codebase contains enormous amounts of information.

An agent rarely needs all of it.

For a question such as:

> Who calls `AuthService.login`?

the agent should not need to:

```text
read files
→ search symbols
→ follow imports
→ parse declarations
→ inspect references
→ reconstruct relationships
```

Prime should already contain the derived knowledge necessary to answer:

```text
AuthService.login
    ← CheckoutController
    ← AdminController
    ← SessionRefreshJob
```

For:

> What does `AuthService.login` depend on?

Prime should already know:

```text
AuthService.login
    → UserRepository.findByEmail
    → PasswordVerifier.verify
    → SessionStore.create
```

For:

> What can `AuthService.login` return or throw?

Prime should expose the derived contract:

```text
returns:
    Session

may throw:
    UserNotFound
    InvalidCredentials
```

The source implementation is not required for these answers.

That is the purpose of Prime.

---

# Prime does not store the code

This is a fundamental constraint.

Prime should **never become a compressed copy of the repository**.

The final representation is not intended to contain:

* source files
* source snippets
* complete ASTs
* reconstructed implementations
* raw file contents
* duplicated syntax
* arbitrary text copied from the repository

Instead, Prime contains **derived knowledge**.

Conceptually:

```text
SOURCE
   │
   │ analyze
   ▼
DERIVED KNOWLEDGE
   │
   │ minimize
   ▼
PRIME
```

Prime is intentionally lossy.

It should not be possible, or even desirable, to reconstruct the original codebase from Prime.

The source repository remains the authority.

Prime is the distilled knowledge layer above it.

---

# The real optimization target

Prime is not a database project.

It is not a graph database project.

It is not a compression project.

It is not a search engine project.

Those may become implementation techniques.

The actual objective is:

> **Maximize the number and quality of agent questions that can be answered without retrieving source code, while minimizing the amount of information, I/O, computation, latency, and context required.**

A useful abstraction is:

```text
                     AGENT KNOWLEDGE
                            ▲
                            │
                            │
                ┌───────────┴───────────┐
                │        PRIME          │
                │                       │
                │   minimum useful     │
                │   representation      │
                └───────────┬───────────┘
                            │
              ┌─────────────┼─────────────┐
              ▼             ▼             ▼
           less I/O    less parsing   less searching
              │             │             │
              └─────────────┼─────────────┘
                            ▼
                     less agent work
```

The important metrics are therefore not simply:

```text
file size
node count
query latency
```

We care about:

```text
questions answerable without source

useful knowledge / byte

useful knowledge / token

useful knowledge / I/O

retrieval latency

computation required

context required

agent tool calls avoided
```

---

# The fundamental research question

Prime ultimately asks:

> **What is the smallest useful representation of a software repository that preserves enough information for a next-generation agent to understand, navigate, modify, and reason about that repository without retrieving the underlying code?**

That question contains several smaller questions:

```text
1. What questions do agents ask?

2. What information is required to answer those questions?

3. What information can be derived from the codebase?

4. Which derived information is actually useful?

5. What is the smallest useful knowledge unit?

6. Which information is redundant from an agent's perspective?

7. How can those units be represented compactly?

8. How can they be retrieved with minimal I/O?

9. How can the representation remain language agnostic?

10. How can it scale from tiny repositories to enormous monorepos?

11. How can it remain useful across different agent architectures?

12. What happens when the available analysis is incomplete or uncertain?
```

---

# Next-generation agents are the consumer

Prime is designed specifically around the architecture of modern and emerging coding agents.

The agent is not a passive reader.

A modern agent typically operates in a loop:

```text
observe
   ↓
reason
   ↓
retrieve
   ↓
reason
   ↓
act
   ↓
observe
   ↓
...
```

Prime therefore needs to optimize for the **information flow through that loop**.

We need to understand:

* model context
* attention
* context windows
* context caching
* tool use
* agent memory
* retrieval loops
* planning
* tool schemas
* structured tool results
* progressive disclosure
* external memory
* agent failure modes
* information overload
* repeated retrieval

The goal is not merely to give an agent more information.

The goal is to give it the **right information in the smallest useful form at the right time**.

---

# Knowledge units

Prime should investigate the smallest independently useful unit of codebase knowledge.

It may turn out to be:

```text
an entity
a fact
a relationship
a contract
a behavioral fact
a dependency
a state transition
a provenance record
```

or some new primitive we have not identified yet.

A conceptual example:

```text
AuthService.login
    ── CALLS ──>
UserRepository.findByEmail
```

Another:

```text
AuthService.login
    ── RETURNS ──>
Session
```

Another:

```text
AuthService.login
    ── MAY_THROW ──>
InvalidCredentials
```

The research must determine whether units like these are sufficient, how they should be combined, and how they can be encoded with minimal overhead.

The concept is intentionally called a **knowledge unit**, not a graph node, because Prime must not assume its final physical representation in advance.

---

# Semantic distillation

Prime is fundamentally a **semantic distillation** problem.

The transformation is:

```text
                 SOURCE
                   │
       ┌───────────┼───────────┐
       ▼           ▼           ▼
    syntax      semantics    structure
       │           │           │
       └───────────┼───────────┘
                   ▼
             DERIVED FACTS
                   │
                   ▼
          REMOVE REDUNDANCY
                   │
                   ▼
         MINIMUM USEFUL FORM
                   │
                   ▼
                 PRIME
```

The important distinction is:

```text
compression:
    make the same information smaller

Prime:
    remove information that is unnecessary
    while preserving what the agent needs
```

Prime therefore requires research into information theory, semantic compression, sufficient statistics, information bottlenecks, rate-distortion, and other ways of understanding what information is actually necessary.

---

# Language agnostic by design

Prime must work across programming languages.

This is a hard requirement.

It should be able to process:

```text
TypeScript
JavaScript
Python
Rust
Go
Java
Kotlin
C
C++
C#
Ruby
PHP
Swift
Scala
Dart
Lua
and others
```

It must also handle **polyglot repositories**:

```text
frontend/
    TypeScript

backend/
    Rust

services/
    Go

native/
    C++

automation/
    Python

infrastructure/
    Terraform
    YAML
```

The representation must preserve meaningful relationships across those boundaries.

For example:

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

The universal model should not attempt to make every language identical.

Instead:

```text
language-specific analysis
          │
          ▼
  universal semantics
          │
          ▼
        PRIME
```

Language-specific frontends may understand different concepts with different levels of precision.

Prime should make those differences explicit.

---

# Capability and uncertainty

Not every language exposes the same amount of static information.

Dynamic dispatch, reflection, macros, metaprogramming, generated code, runtime loading, and other features can make some facts impossible to establish statically.

Prime therefore needs to distinguish knowledge such as:

```text
EXACT
DERIVED
INFERRED
UNKNOWN
```

and, where appropriate, preserve:

```text
confidence
provenance
analysis source
source location
revision
```

Example:

```text
AuthService.login
    CALLS
UserRepository.findByEmail

confidence:
    exact

evidence:
    static symbol resolution
```

versus:

```text
PluginManager
    MAY_CALL
PaymentProvider

confidence:
    inferred

reason:
    dynamic dispatch
```

Prime must never pretend uncertain knowledge is exact.

---

# Small and enormous codebases

Prime must work across radically different repository sizes.

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

A tiny project should not require a heavyweight infrastructure stack.

A huge repository should not require loading the entire knowledge representation into memory.

The design must investigate:

* streaming analysis
* incremental processing
* parallel analysis
* partial loading
* memory mapping
* immutable structures
* compact indexing
* sharding where useful
* content addressing
* caching
* incremental invalidation

Scale is not an optimization phase to add later.

It is part of the core problem.

---

# Prior art

Prime should build on existing research rather than pretending the field starts here.

Important systems include:

### SCIP

A language-agnostic source-code indexing protocol covering symbols, definitions, references, and implementations.

https://github.com/sourcegraph/scip

### LSIF

A persistent representation of language-server information designed to make code intelligence available without repeatedly running the language server.

https://github.com/microsoft/lsif-node

### Code Property Graph / Joern

A rich program representation combining multiple forms of program structure and analysis.

https://github.com/joernio/joern

https://cpg.joern.io/

### Tree-sitter

An incremental, error-tolerant parsing ecosystem supporting many languages.

https://github.com/tree-sitter/tree-sitter

### Aider repository map

A practical demonstration that a compact structural map of a repository can provide substantial value to an LLM while remaining within a token budget.

https://aider.chat/2023/10/22/repomap.html

### Agent-oriented code indexes

Projects such as codebase-index and other repository intelligence systems explore hybrid combinations of symbol indexes, lexical search, relationships, embeddings, and context assembly.

Prime should study all of these carefully, including their source code and architectural tradeoffs.

---

# Research beyond traditional code tooling

Prime deliberately has a very wide research scope.

The optimal solution may come from outside traditional code indexing.

We will investigate:

## Information theory

* entropy
* information bottleneck
* sufficient statistics
* minimum description length
* rate-distortion
* semantic compression
* information-preserving transformations

## Graph and data compression

* delta encoding
* variable-length integers
* compressed adjacency
* grammar compression
* succinct graphs
* WebGraph-style techniques
* Elias-Fano
* rank/select
* bit packing

## Search and indexing

* inverted indexes
* FSTs
* minimal perfect hashing
* learned indexes
* approximate indexes
* Bloom filters
* quotient filters
* Roaring bitmaps

## Distributed systems

* content addressing
* Merkle trees
* Merkle DAGs
* peer-to-peer distribution
* deduplication
* distributed caching
* synchronization
* CRDTs

## Cryptography

* integrity
* provenance
* signed knowledge
* commitments
* Merkle proofs
* searchable encryption
* privacy-preserving retrieval
* zero-knowledge techniques

Cryptography is not being researched because encryption is inherently faster.

It is being researched because **trust, provenance, identity, deduplication, and distributed knowledge verification** may materially improve the overall system.

## Systems

* memory mapping
* page cache behavior
* SSD / NVMe I/O
* CPU cache locality
* SIMD
* zero-copy access
* concurrency
* immutable data structures
* persistent data structures

## Agent architecture

* context management
* memory
* tool use
* model attention
* context caching
* retrieval loops
* agent planning
* context selection
* long-context behavior
* structured tool outputs

Prime intentionally does not restrict its research to technologies traditionally associated with code intelligence.

---

# The artifact

The eventual Prime product is expected to produce a **single logical knowledge artifact** representing a codebase.

The physical representation is deliberately undecided during research.

It could eventually use:

* one binary file
* a custom format
* memory-mapped structures
* compressed arrays
* content-addressed blocks
* specialized indexes
* another structure not yet identified

The constraint is the outcome, not the implementation:

> **The artifact should expose the maximum useful codebase knowledge with the minimum retrieval cost.**

The final representation must never require the original source to answer questions that can already be derived from Prime.

---

# What Prime should answer

Prime should eventually support questions such as:

```text
Where is X?

What is X?

What does X represent?

What does X depend on?

What depends on X?

Who calls X?

What does X call?

What implements X?

What does X implement?

What references X?

What uses X?

What tests X?

What configuration affects X?

What is the architecture around X?

What components are connected to X?

What would be affected if X changes?

What are the contracts of X?

What are the known behaviors of X?

What are the known failure modes of X?

What is the smallest context required to understand X?
```

These are examples, not yet the final API.

The research must discover the actual question space of next-generation coding agents.

---

# Prime should be question-oriented

Traditional systems often begin with:

```text
nodes
edges
rows
documents
chunks
vectors
```

Prime should begin with:

```text
agent question
        ↓
information requirement
        ↓
minimum knowledge
        ↓
retrieval
```

The underlying representation is secondary.

The end goal is an efficient answer.

---

# Retrieval without source access

The defining property of Prime is:

```text
Agent question
      ↓
Prime
      ↓
answer
```

not:

```text
Agent question
      ↓
Prime
      ↓
source lookup
      ↓
parse
      ↓
search
      ↓
answer
```

Prime may retain provenance pointing back to the source, but provenance is not the answer.

The source is the authority and fallback.

Prime is the precomputed knowledge.

---

# Deliberately lossy

Prime should be intentionally non-reversible.

The transformation:

```text
source → prime
```

should discard information that does not contribute useful knowledge for agents.

This can include:

* formatting
* repeated syntax
* implementation details that are irrelevant to known questions
* redundant representations
* incidental identifiers
* non-semantic text
* source-level structure that does not improve retrieval

This is a design advantage, not a limitation.

---

# Research methodology

Prime follows one rule:

> **Do not design the answer before understanding the problem.**

Research should use:

* primary documentation
* official specifications
* source repositories
* academic papers
* implementation analysis
* benchmarks
* experiments
* real repositories
* agent evaluations

Every major conclusion should distinguish between:

```text
FACT
OBSERVATION
EXPERIMENTAL RESULT
HYPOTHESIS
INFERENCE
OPEN QUESTION
```

Do not present an architectural preference as an established fact.

Do not choose a technology because it is familiar.

Do not preserve a design simply because we implemented it first.

---

# Research → documentation → Prime

The repository itself is deliberately organized into three layers.

```text
                  RESEARCH
                     │
                     ▼
              RESEARCH FINDINGS
                     │
                     ▼
                   DOCS
                     │
                     ▼
             FUTURE PRIME PRODUCT
                     │
                     ▼
                    ACC
```

### `research/`

Contains the external knowledge and experiments.

It answers:

> What do we know?

### `docs/`

Contains the technical conclusions derived from the research.

It answers:

> What should Prime be?

### `.acc/`

Contains the project context, standards, agent roles, and constraints required to work on Prime effectively.

It answers:

> How should agents work on Prime?

---

# Repository structure

The repository intentionally begins with only two files:

```text
Prime/
├── README.md
└── init-prompt.md
```

`init-prompt.md` is the bootstrap specification for the research agent.

After initialization, the repository is expected to evolve into:

```text
Prime/
│
├── README.md
├── init-prompt.md
│
├── research/
│   ├── agents/
│   ├── codebase-analysis/
│   ├── prior-art/
│   ├── representation/
│   ├── information-theory/
│   ├── compression/
│   ├── indexing/
│   ├── retrieval/
│   ├── storage/
│   ├── distributed/
│   ├── cryptography/
│   ├── systems/
│   ├── languages/
│   ├── experiments/
│   ├── benchmarks/
│   └── references/
│
├── docs/
│   ├── research-synthesis/
│   ├── requirements/
│   ├── representation/
│   ├── retrieval/
│   ├── architecture/
│   └── decisions/
│
└── .acc/
    ├── config.yaml
    ├── config/
    └── agents/
```

Nothing in this structure is considered final until the research justifies it.

---

# Research phases

The research should progress in this order.

```text
PHASE 1
Understand next-generation agents
            │
            ▼
PHASE 2
Understand codebase information
            │
            ▼
PHASE 3
Study existing representations
            │
            ▼
PHASE 4
Discover the minimum useful knowledge unit
            │
            ▼
PHASE 5
Research representation and compression
            │
            ▼
PHASE 6
Research retrieval and agent interaction
            │
            ▼
PHASE 7
Research scale and language agnosticism
            │
            ▼
PHASE 8
Build isolated experiments
            │
            ▼
PHASE 9
Benchmark agent tasks
            │
            ▼
PHASE 10
Synthesize findings
            │
            ▼
PHASE 11
Write the Prime specification
            │
            ▼
PHASE 12
Configure ACC
            │
            ▼
PHASE 13
Build Prime
```

The order matters.

**Implementation is the final phase, not the first.**

---

# Language coverage requirements

Prime should eventually be evaluated against:

### Small repositories

* small TypeScript project
* small Python project
* small Rust project
* small Go project

### Large repositories

* large TypeScript monorepo
* large Java monorepo
* large Rust workspace
* large C/C++ codebase
* large Python ecosystem repository

### Polyglot repositories

Repositories containing multiple languages with relationships across boundaries.

### Difficult repositories

Repositories containing:

* generated code
* macros
* reflection
* dynamic dispatch
* metaprogramming
* build-generated sources
* unusual module systems
* large dependency graphs

The goal is not merely:

> "support many parsers."

The goal is:

> **derive a useful universal knowledge representation across fundamentally different programming models.**

---

# Scale requirements

Prime must be designed to investigate repositories ranging from:

```text
tiny
small
medium
large
very large
monorepo
polyglot monorepo
```

The representation should not assume:

```text
all code fits in memory
all code can be analyzed in one pass
all relationships are local
all languages behave statically
```

Potential techniques include:

* streaming
* parallel analysis
* incremental analysis
* partial loading
* memory mapping
* compact indexes
* content addressing
* immutable snapshots
* distributed analysis
* caching

The appropriate combination must be established through research and measurement.

---

# Agent efficiency

The strongest Prime benchmark should ultimately be based on **agent tasks**, not storage benchmarks alone.

Compare, on identical repositories and tasks:

```text
raw filesystem access

repository maps

existing code indexes

graph retrieval

hybrid retrieval

Prime
```

Measure:

```text
task success
time
tool calls
bytes transferred
tokens exposed
retrieval precision
retrieval recall
context redundancy
agent corrections
source accesses required
```

The central question is:

> **How much useful codebase understanding can an agent obtain before it needs to inspect source?**

---

# The ultimate benchmark

A successful Prime artifact should dramatically reduce:

```text
source reads
searches
parsing
relationship discovery
context reconstruction
agent tool calls
```

while preserving:

```text
correctness
useful context
architectural understanding
relationship accuracy
agent task performance
```

The ideal system is not merely smaller.

It makes the codebase **legible**.

---

# Relationship to ACC

Prime is designed to work alongside [Agent Code Context (ACC)](https://github.com/EnzoVezzaro/agents-code-context).

ACC and Prime solve different problems.

ACC provides project-level agent context:

```text
standards
architecture
rules
contracts
workflows
project knowledge
```

Prime investigates codebase-derived knowledge:

```text
symbols
relationships
dependencies
architecture
contracts
behavior
impact
```

Conceptually:

```text
                    CODEBASE
                       │
              ┌────────┴────────┐
              ▼                 ▼
             ACC              PRIME
              │                 │
      project knowledge   derived knowledge
              │                 │
              └────────┬────────┘
                       ▼
                     AGENT
```

Prime should eventually become a lower-level knowledge layer that ACC can consume.

ACC should not dictate Prime's internal representation.

Prime should not replace ACC's project-context model.

---

# What Prime is not

Prime is not:

* a source-code compressor
* a code archive
* a compiler
* a programming language
* a language server
* a graph database
* a vector database
* an embedding store
* a documentation generator
* an IDE
* an agent
* a replacement for Git
* a replacement for source code

Prime is also not required to use:

* graphs
* databases
* embeddings
* cryptography
* P2P
* memory mapping
* custom binary formats

Those are research areas.

The final system should use whatever combination of techniques best satisfies the objective.

---

# Status

**Research / Early Stage**

At this stage:

* no final Prime format exists
* no final graph model exists
* no final storage design exists
* no final retrieval API exists
* no final compression algorithm exists
* no final language model exists

Those decisions are intentionally postponed.

The current goal is to understand the problem deeply enough to make them responsibly.

---

# Open source

Prime is open research.

The value of the project is not only the eventual implementation.

The research itself should remain useful to other people working on:

* code intelligence
* static analysis
* code search
* AI agents
* programming languages
* databases
* storage engines
* compression
* distributed systems
* information retrieval
* machine-readable software representations

Useful contributions include:

* papers
* technical references
* repositories
* implementation analysis
* benchmarks
* experiments
* counterexamples
* alternative approaches
* failed approaches
* new ideas
* corrections

A finding that disproves a promising idea is useful.

A benchmark showing that a "clever" approach is actually slower is useful.

A technology from an unrelated field that turns out to solve part of this problem is especially useful.

---

# The Prime principle

Everything in this repository should ultimately be measured against one sentence:

> **Prime exists to minimize the information an agent must retrieve from a codebase while maximizing the agent's ability to understand and reason about that codebase.**

Not smaller files for their own sake.

Not faster graphs for their own sake.

Not better databases for their own sake.

Not more search results.

**Less information retrieved.
More understanding achieved.**

That is Prime.
