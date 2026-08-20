# Prime Agent Representation — Tracking Document

**Status**: Design Phase  
**Created**: 2025  
**Owner**: Research Team  
**Last Updated**: 2025-08-20

---

## Executive Summary

This document tracks the refactoring of Prime around **Prime Agent Representation (PAR)** — a new explicit feature that separates *how Prime stores knowledge* from *how Prime communicates knowledge to an agent model*.

The core insight: **Prime should not merely make software smaller; it should make software expressible in the most efficient form for the model that has to reason about it.**

---

## Core Research Findings (Evidence-Based)

### 1. Tokenization Affects Reasoning
- Poor token boundaries can interfere with symbolic reasoning
- Atomically aligned representations can improve structured reasoning
- Source: arXiv:2505.14178 "Tokenization Constraints in LLMs"

### 2. Raw Compiler IR ≠ Good for LLMs
- LLMs can parse IR syntax but struggle with control-flow/execution semantics
- Source: arXiv:2502.06854 "Can LLMs Understand Intermediate Representations?"
- **Conclusion**: Don't use SSA/CFG/LLVM-IR as agent representation

### 3. Structured Tool Outputs Are Native Model Interface
- OpenAI Structured Outputs / MCP support constrained decoding + output schemas
- Prime should expose: `semantic operation → compact structured result`
- Not: giant textual document dump

### 4. Semantic Compression > Syntactic Compression
- Prompt compression research shows large reductions possible while preserving task performance
- Token Sugar explores token-efficient shorthand for source code
- **Prime opportunity**: We're not compressing source — we're removing source entirely and expressing only derived semantic information

---

## Architecture: Four-Layer Refactor

```
CODEBASE
    │
    ▼
PRIME DERIVATION
    │
    ▼
PRIME KNOWLEDGE CORE
    │
    ├─────────────────────┐
    ▼                     ▼
binary artifact        query engine
                          │
                          ▼
                   AGENT REPRESENTATION
                          │
                          ▼
                         LLM
```

### New Critical Component: Agent Representation Compiler

**Not a traditional compiler** — a **projection layer**:

```
Prime Knowledge
    +
Agent Question
    +
Model/Tokenizer Profile
    ↓
Minimal Agent Representation
```

---

## Dual Representation Strategy

### Canonical Representation (Internal)
Optimized for: storage, I/O, deterministic lookup, indexing, compression, incremental updates
```
entities
relationships
types
contracts
architecture
provenance
confidence
```

### Agent Representation (Model-Facing)
Optimized for: model recognition, token efficiency, reasoning, minimal context, tool-result size
```
semantic facts
compact notation
short aliases
typed relations
```

**Critical distinction**: These are separate representations for separate consumers.

---

## PAR — Prime Agent Representation

### Design Principles
- **Semantic anchors preserved**: Use words the model already knows (`CALLS`, `RETURNS`, `IMPLEMENTS`, `DEPENDS_ON`)
- **Not "LLM machine language"**: Avoid opaque encodings like `A17 → B42`
- **Compression with semantic grounding**: First establish semantics, then compress repetition
- **Query-shaped**: Return only what the question needs, not the entire knowledge graph

### Example Evolution

**Full semantic form:**
```
@FN AuthService.login
  IN email:string password:string
  OUT Session
  CALL UserRepository.findByEmail
  CALL PasswordVerifier.verify
  CALL SessionStore.create
  THROW UserNotFound InvalidCredentials
  TEST AuthService.login.test
```

**Compressed with aliases:**
```
$1 = AuthService.login
$2 = UserRepository
$3 = Session

$1 CALL $2.findByEmail,PasswordVerifier.verify,SessionStore.create
$1 OUT $3
$1 THROW UserNotFound,InvalidCredentials
```

---

## Query-Shaped Projection (Critical)

**Don't compile entire artifact.** Instead:

```
Agent Question
    ↓
Prime Query Planner
    ↓
Minimum Knowledge Slice
    ↓
PAR Projection
    ↓
LLM
```

**Example:**
> "Who calls `AuthService.login`?"
```
AuthService.login
CALLER CheckoutController.handle
CALLER AdminController.login
CALLER SessionRefreshJob.run
```
NOT the entire codebase.

---

## Semantic Projection Operation

New internal operation:
```
project(knowledge, query, target=agent)
```

**Example:**
```
impact(User.email)
    ↓
User.email → UserSerializer, UserValidator, AuthService, UserRepository
    ↓
User.email
  AFFECTS UserSerializer
  AFFECTS UserValidator
  AFFECTS AuthService
  AFFECTS UserRepository
```

---

## Model-Adaptive Representation

```
Prime Knowledge
    │
    ▼
Representation Planner
    │
┌────┼────┐
▼    ▼    ▼
GPT  Gemini Claude
│    │     │
▼    ▼     ▼
PAR  PAR   PAR
```

Semantic representation stays identical; only serialization/projection strategy changes.

---

## Tokenizer-Aware Rendering

Renderer measures:
```
representation → target tokenizer → token count
```

Chooses among equivalent encodings:
```
CALLS | calls | → | C
```

Benchmarks token cost of vocabulary across models.

---

## Stable Vocabulary (Finite, Controlled)

```
DEF, CALL, REF, IMP, EXT, RET, ARG, TYPE, READ, WRITE, DEP, TEST, THROW, CONFIG, LOC
```

Properties:
- Finite
- Documented
- Stable
- Semantically unambiguous
- Tokenizer-tested

---

## Familiar Semantic Words > Arbitrary Symbols

Prefer:
```
CALL, RETURN, DEP, IMPLEMENTS, THROW, TEST
```
Over:
```
→, ←, #, $, @, %
```
Unless experiments prove shorter symbols improve model performance.

---

## Local Aliases (Dictionary Compression)

```
$R = UserRepository
$R.findById
$R.save
$R.delete
$R.validate
```

First occurrence establishes semantics; rest becomes cheap.

---

## Canonical Ordering (Deterministic)

```
ENTITY
SIGNATURE
TYPE
CALLS
CALLEES
CALLERS
DEP
TEST
IMPACT
```

Benefits: caching, reproducibility, meaningful diffs, stable token sequences, prefix sharing, model familiarity.

---

## Prefix Caching (Prompt Caching)

Stable prefix + small changing query result:
```
PRIME/1
SCHEMA/1
ENTITY RULES
RELATION RULES
VOCABULARY
...
QUERY RESULT
```

Only last section changes → prefix caching opportunities.

---

## MCP Should Expose Projection, Not Artifact

```json
{
  "format": "par-1",
  "coverage": "complete",
  "content": "AuthService.login CALLS UserRepository.findByEmail ..."
}
```

MCP supports structured tool results + output schemas.

---

## Refined Architecture

```
CODEBASE
    │
    ▼
PRIME DERIVER
    │
    ▼
PRIME KNOWLEDGE
    │
    ▼
QUERY PLANNER
    │
    ▼
MINIMUM KNOWLEDGE SLICE
    │
    ▼
AGENT REPRESENTATION
    │
┌───────┴───────┐
▼               ▼
token optimizer  schema
    │              │
    └──────┬──────┘
           ▼
          MCP
           │
           ▼
          AGENT
```

---

## Removed Assumptions

❌ "Prime's graph is what the agent reads"  
✅ "Prime's graph is what Prime queries; agent receives minimal semantic projection"

❌ "Binary Prime artifact needs to be human/LLM readable"  
✅ "Artifact optimized for Prime; projection optimized for model"

---

## New Feature Set

| Feature | Purpose |
|---------|---------|
| `Prime Knowledge Core` | Canonical derived knowledge |
| `Prime Query Engine` | Finds minimum knowledge needed |
| `Prime Agent Representation` | Canonical semantic notation |
| `Prime Projection Engine` | Converts knowledge → agent views |
| `Prime Token Optimizer` | Measures tokenization, selects encoding |
| `Prime MCP` | Exposes projections to agents |

---

## Benchmark Transformation

### Old Benchmark
> "How fast/small is Prime?"

### New Benchmark
> "How should machine-derived software knowledge be encoded for an LLM?"

### Test Matrix
| Representation | Tokens | Accuracy | Latency | Source-Free Rate |
|----------------|--------|----------|---------|------------------|
| Natural Language | | | | |
| JSON | | | | |
| YAML | | | | |
| Graph Triples | | | | |
| Compact Triples | | | | |
| PAR | | | | |
| Tokenizer-Optimized PAR | | | | |

### Test Conditions
- Same: repository, questions, model, temperature, tool availability
- Measure: accuracy, precision, recall, tokens, latency, source accesses, tool calls, task success

---

## Ablation Framework

Test knowledge quality vs. artifact size:
```
100% knowledge → 1.4 MB
 98% knowledge → 700 KB
 95% knowledge → 300 KB
```

---

## Incremental & Mutation Benchmarks

### Incremental
```
initial → 1 changed file
initial → 10 changed files
initial → 1% changed
initial → 10% changed
```
Measure: rebuild time, incremental time, artifact delta, affected entities/relations

### Mutation
```
commit A → Prime A
commit B (rename/remove/add/change/move) → Prime B
```
Test: Does Prime B reflect new truth? Does Prime A avoid claiming B's knowledge?

---

## Implementation Roadmap

### Phase 1: Infrastructure ✅
- [x] Benchmark infrastructure (schema, corpus, harness, CI)
- [x] PAR schema design
- [x] Query planner framework
- [x] Corpus preparation (`prime prepare`)

### Phase 2: PAR Core (In Progress)
- [ ] PAR schema formalization
- [ ] Projection engine (knowledge → slice → PAR)
- [ ] Token-aware renderer
- [ ] Vocabulary + alias system

### Phase 3: Model Adaptation
- [ ] Tokenizer benchmarking across models
- [ ] Model-specific renderers
- [ ] Token cost measurement infrastructure

### Phase 4: Benchmarking & Validation
- [ ] Full representation matrix benchmark
- [ ] Source-free QA benchmark
- [ ] Ablation framework (minimal/normal/compressed)
- [ ] Incremental/mutation benchmarks
- [ ] Source-free agent A/B benchmark

---

## Open Questions / Risks

| Risk | Mitigation |
|------|------------|
| Model may not understand PAR shorthand | Benchmark against natural language baseline |
| Tokenizer differences cause divergence | Test across GPT/Claude/Gemini/open models |
| PAR may not compress enough | Ablation: test minimal/normal/compressed |
| Query planner may miss context | Compare full-context vs. sliced accuracy |
| Token optimization may hurt semantics | Rule: compression only when meaning preserved |

---

## Acceptance Criteria for v1.0

- [ ] PAR schema formalized and documented
- [ ] Projection engine produces correct slices for 10+ question types
- [ ] Token optimizer selects best encoding for GPT/Claude/Gemini
- [ ] Full benchmark matrix executed (7 representations × 4 models × 5 repos)
- [ ] Ablation results published
- [ ] Source-free accuracy > baseline JSON/graph
- [ ] Prefix caching demonstrably reduces prefill
- [ ] MCP exposes PAR projection with structured schema

---

## Related Documents

- `SPECS/01-codebase-knowledge/par-spec.md` (to be created)
- `SPECS/06-agent-representation/` (directory to be created)
- `benchmarks/README.md` — benchmark methodology
- `specs/agent-native-interface.md` — existing agent interface spec

---

## Change Log

| Date | Author | Change |
|------|--------|--------|
| 2025-08-20 | Research Team | Initial design document created from research findings |

---

## References

1. arXiv:2505.14178 — Tokenization Constraints in LLMs
2. arXiv:2502.06854 — Can LLMs Understand Intermediate Representations?
3. OpenAI Structured Outputs — https://openai.com/index/introducing-structured-outputs-in-the-api/
3. arXiv:2403.12968 — LLMLingua-2
4. arXiv:2512.08266 — Token Sugar
5. Model Context Protocol — https://modelcontextprotocol.io/