# Git as Content-Addressed Data System — Prior Art Analysis

## Confidence Legend
- **FACT** — Verified by primary source (Git source, Pro Git book, documentation)
- **OBSERVATION** — Directly observed from source behavior or documented mechanics
- **HYPOTHESIS** — Proposed explanation requiring validation
- **INFERENCE** — Deduced from evidence, marked as such
- **OPEN QUESTION** — Explicitly unknown, needs research

---

## 1. Core Model: Content-Addressable Filesystem

### 1.1 Fundamental Abstraction (FACT)
Git is **"a content-addressable filesystem"** — a simple key-value data store where *content determines identity* [Pro Git 10.2](https://git-scm.com/book/en/v2/Git-Internals-Git-Objects).

```
Key = SHA-1(header + content)  where header = "type size\0"
Value = zlib-compressed(content + header)
```

### 1.2 Object Types (FACT)
| Type | Purpose | Content |
|------|---------|---------|
| **blob** | File content | Raw bytes |
| **tree** | Directory snapshot | List of (mode, type, name, SHA-1) entries |
| **commit** | Snapshot + metadata | tree SHA-1, parent(s), author, committer, message |
| **tag** | Named reference | Object SHA-1, tagger, message |

### 1.3 Content Addressing Properties (FACT)
- **Immutability**: Same content → same SHA-1 → same object
- **Deduplication**: Identical content stored once (e.g., two files with same content share blob)
- **Integrity**: SHA-1 collision = corruption detection
- **Merkle Structure**: Trees reference blobs/subtrees by SHA-1; commits reference trees → **cryptographic snapshot chain**

---

## 2. Storage Layers

### 2.1 Loose Objects (FACT)
- One file per object: `.git/objects/aa/bbbbbb...` (first 2 chars = directory)
- zlib-compressed individually
- Simple, but **inefficient at scale** (thousands of small files, no delta compression)

### 2.2 Packfiles (FACT)
- **Binary format** combining multiple objects [Pro Git 10.4](https://git-scm.com/book/en/v2/Git-Internals-Packfiles)
- **Delta compression**: Store base object + deltas (not full copies)
- **Index (.idx)**: Offsets into packfile for O(log n) object lookup
- **Created by**: `git gc`, `git repack`, push to remote
- **Repackable**: Can be rewritten anytime to improve delta chains

### 2.3 Packfile Delta Strategy (OBSERVATION)
```
Repo.rb v1 (22KB) → stored as base
Repo.rb v2 (22KB) → stored as 9-byte delta from v1
```
- **Heuristic**: Similar name + similar size → delta candidate
- **Direction**: Newer version stored intact; older as delta (optimizes for recent access)
- **Chain length**: Limited (default max 50? configurable)
- **Threads**: `Delta compression using up to 8 threads` (parallel delta computation)

---

## 3. Snapshots vs Deltas: The Git Insight

### 3.1 Snapshot Model (FACT)
> *"Git stores content in a manner similar to a UNIX filesystem, but a bit simplified. All the content is stored as tree and blob objects, with trees corresponding to UNIX directory entries and blobs corresponding more or less to inodes or file contents."* — Pro Git 10.2

Each commit = **complete tree snapshot** (not diff from parent). Trees are **content-addressed** → identical subtrees shared automatically.

### 3.2 Delta as Storage Optimization Only (INFERENCE)
- **Logical model**: Snapshots (trees)
- **Physical model**: Deltas in packfiles
- **Key**: Deltas are **transparent to the object model** — `git cat-file` returns full content regardless of storage
- **Repacking** can change delta chains without changing object IDs

### 3.3 Reachability (FACT)
Objects are **reachable** if referenced from a ref (branch, tag) or another reachable object.
- `git gc` packs **only reachable** objects
- **Dangling objects** (unreferenced) remain as loose objects until pruned
- **Reachability bitmap** (`.idx` extension) accelerates `git push`/`fetch` negotiation

---

## 4. Incremental Operations

### 4.1 Index (Staging Area) (FACT)
- **File**: `.git/index` — binary format, maps path → (mode, SHA-1, stage, stat info)
- **Role**: Accumulates changes → `git write-tree` produces tree object
- **Stat cache**: `mtime`, `size` for fast "is file changed?" checks
- **Incremental**: `git add` updates only changed entries

### 4.2 Incremental Packfile Creation (OBSERVATION)
```
git add file → loose blob
git commit → tree + commit objects (loose)
git gc → packfiles with delta compression
```
- **Loose → Pack** transition is batched, not per-operation
- **Tradeoff**: Write amplification (loose objects) vs. read performance (packed)

### 4.3 Partial Clone / Shallow (FACT)
- `--depth=N` — truncate history
- `--filter=blob:none` — omit blobs, fetch on demand
- **Relevance to Prime**: Prime could support **shallow knowledge graphs** (recent changes only) or **lazy loading** of semantic artifacts

---

## 5. Transfer Protocols & Negotiation

### 5.1 Smart HTTP / SSH (FACT)
- **Negotiation**: Client sends `have`/`want` lists of commit SHA-1s
- **Server computes** minimal packfile containing only missing objects
- **Thin packs**: Send deltas against objects client already has (further compression)

### 5.2 Bundle Files (FACT)
- `git bundle create` — single file containing packfile + refs
- **Portable**: Can be emailed, copied via USB
- **Relevance**: Prime could export **knowledge graph bundles** for offline/air-gapped use

---

## 6. Comparison: Git Object Model vs Prime Knowledge Artifacts

| Dimension | Git Objects | Prime Knowledge Graph |
|-----------|-------------|----------------------|
| **Identity** | Content hash (SHA-1/256) | Content hash (semantic derivation) |
| **Immutability** | Absolute (content-addressed) | Absolute (semantic snapshot) |
| **Composition** | Tree → (blob\|tree)* | KnowledgeGraph → (Entity, Relation)* |
| **Delta Storage** | Packfile (physical only) | Semantic delta (logical + physical) |
| **Snapshots** | Commit → tree → blobs | PrimeEnvelope → snapshot → entities |
| **Incremental** | Index → tree → commit | Incremental analysis (planned) |
| **Deduplication** | Automatic (same content = same hash) | Semantic equivalence (harder) |
| **Reachability** | Ref → commit → tree → blob | Query → entity → relations |
| **Transfer** | Packfile negotiation | MCP tools / bundle export |
| **Verification** | `git fsck` (SHA-1 + connectivity) | Provenance + cryptographic proofs (planned) |

---

## 7. What Prime Should BORROW

### 7.1 Content-Addressed Semantic Artifacts (FACT → BORROW)
```
Source snapshot (Git commit) 
  → Semantic derivation (parsing, analysis) 
  → Immutable Prime snapshot (content-addressed)
```
- **Prime artifact hash** = `H(derivation_algorithm_version + source_snapshot_hash + config)`
- **Identical source + identical analyzer = identical Prime artifact** — enables caching, sharing, verification
- **Merkle DAG**: Prime snapshot → entities → relations → all content-addressed

### 7.2 Snapshot + Delta Duality (FACT → BORROW)
- **Logical**: Prime stores **semantic snapshots** per source version (like Git commits → trees)
- **Physical**: **Delta compression** between snapshots (like packfiles)
- **Semantic deltas**: "Entity X changed signature", "Relation Y added" — not byte deltas
- **Repacking**: Periodic recomputation of optimal delta chains

### 7.3 Index as Staging Area (FACT → BORROW)
- **Prime Index** = staging area for incremental analysis
- Track: `file_path → (source_hash, entity_set_hash, relation_set_hash, mtime)`
- `prime build --incremental` reads index → computes diff → updates only affected entities

### 7.4 Reachability for Garbage Collection (FACT → BORROW)
- **Root set**: Current HEAD snapshot + recent snapshots (configurable retention)
- **Traverse**: Snapshot → entities → relations
- **Collect**: Unreachable entities/relations (from deleted files, abandoned branches)
- **Packfile analogy**: Compact storage by rewriting reachable subgraph

### 7.5 Bundle / Transfer Protocol (FACT → BORROW)
- `prime export --bundle` → single file with snapshots + index
- `prime import --bundle` → verify hashes, integrate
- **Negotiation**: "I have snapshot H1, H2; give me deltas to H3"
- **Thin bundles**: Deltas against known snapshots

### 7.6 Stat-Based Fast Path (OBSERVATION → BORROW)
- Git index stores `mtime`/`size` → `git status` avoids re-hashing unchanged files
- Prime index stores `source_hash` → `prime check` avoids re-parsing unchanged files
- **Fast path**: `stat()` → compare mtime/size → if unchanged, trust index

---

## 8. What Prime Should NOT Borrow

### 8.1 SHA-1 for Content Addressing (FACT → AVOID)
- **SHA-1 broken** for collision resistance (SHAttered 2017)
- Git migrating to **SHA-256** (v2.29+)
- Prime should use **SHA-256 or BLAKE3** from start

### 8.2 Heuristic Delta Matching (OBSERVATION → AVOID)
- Git: "similar name + similar size" → delta candidate
- **Semantic deltas need semantic similarity**, not syntactic heuristics
- Prime: Delta = **explicit semantic change** (entity added/removed/modified) with structured representation

### 8.3 Loose Object Phase (OBSERVATION → AVOID)
- Git writes loose objects first, packs later (batch)
- Prime should **write directly to packed format** (append-only log + periodic compaction)
- Avoids loose-object filesystem overhead

### 8.4 Single Packfile per Repository (OBSERVATION → AVOID)
- Git: One packfile (or few) per repo
- Prime: **Multiple snapshots** → multiple packfiles / segmented storage
- **Time-travel queries**: "Show me the graph at commit X" requires snapshot isolation

### 8.5 No Semantic Awareness in Storage (INFERENCE → AVOID)
- Git packfiles are **content-agnostic** (compress bytes)
- Prime storage should be **semantically aware**: entity boundaries, relation types, query patterns
- Enables: partial loads, predicate pushdown, semantic compression

---

## 9. Prime-Specific Adaptation: Semantic Snapshot Model

### 9.1 Proposed Model (HYPOTHESIS)
```
Source Snapshot (Git commit hash)
  │
  ├─► Parser Version + Config Hash
  │
  ├─► Semantic Derivation
  │     ├─► Entities (content-addressed: H(kind + qualified_name + signature + span))
  │     ├─► Relations (content-addressed: H(src + dst + type + metadata))
  │     └─► Index (file → entity refs, symbol → entity refs)
  │
  └─► Prime Snapshot Hash = H(derivation_hash + entities_hash + relations_hash + index_hash)
```

### 9.2 Incremental Semantic Delta (HYPOTHESIS)
```
Prime Snapshot N+1 = Prime Snapshot N + Semantic Delta
Semantic Delta = {
  added_entities:    EntityRef[],
  removed_entities:  EntityRef[],
  modified_entities: (EntityRef, EntityDiff)[],
  added_relations:   RelationRef[],
  removed_relations: RelationRef[],
}
```
- **EntityDiff**: Structured (signature_changed, span_changed, kind_changed, ...)
- **Delta chain**: Snapshot → Delta → Delta → ... (like Git packfile chains)
- **Base snapshot**: Full materialization every N deltas (configurable)

### 9.3 Query-Time Materialization (HYPOTHESIS)
- `prime query` on snapshot N:
  1. Load base snapshot (mmap)
  2. Apply delta chain (in memory, streaming)
  3. Answer query
- **Parallel**: Multiple queries share base snapshot (mmap = zero-copy sharing)

---

## 10. Open Questions

1. **OPEN QUESTION**: What is the optimal **snapshot granularity**? Per-commit? Per-push? Per-day? Per-analyzer-version?

2. **OPEN QUESTION**: **Semantic equivalence** for deduplication — when are two entities "the same" across snapshots? Qualified name? Signature? Body hash? Structural equivalence?

3. **OPEN QUESTION**: **Delta chain length limit** — Git limits to ~50. What's the right limit for semantic deltas? Tradeoff: query latency vs. storage.

4. **OPEN QUESTION**: **Cross-snapshot queries** — "How did this API evolve?" Requires joining across snapshots. How to index?

5. **OPEN QUESTION**: **Partial snapshot loading** — Can we load only entities relevant to a query (like Git's `git cat-file` loads one object)? Requires entity-level addressing in packfile.

6. **OPEN QUESTION**: **Concurrent writers** — Multiple agents updating Prime index. Git uses locks/refs. Prime needs **CRDT or OT** for distributed knowledge (research area 18).

7. **OPEN QUESTION**: **Garbage collection policy** — Retain all snapshots? Last N? Only tagged? Reachability from "active" branches?

8. **OPEN QUESTION**: **Verification** — `prime fsck` equivalent: verify entity/relation hashes, check referential integrity, detect corruption.

9. **OPEN QUESTION**: **Storage format** — Custom binary vs. existing (SQLite, RocksDB, LMDB, Parquet)? Git uses custom packfile. Prime may need custom for semantic query patterns.

10. **OPEN QUESTION**: **Analyzer versioning** — If parser/analyzer changes, all snapshots invalidated. How to track? `derivation_algorithm_version` in snapshot hash.

---

## 11. Evidence Summary

| Claim | Evidence | Confidence |
|-------|----------|------------|
| Content-addressable filesystem | Pro Git 10.2, `git hash-object` | FACT |
| Four object types (blob, tree, commit, tag) | Pro Git 10.2 | FACT |
| SHA-1(header + content) identity | Pro Git 10.2, Ruby demo | FACT |
| zlib compression per object | Pro Git 10.2 | FACT |
| Packfiles with delta compression | Pro Git 10.4, `git verify-pack` output | FACT |
| Delta: newer intact, older as delta | Pro Git 10.4 example | OBSERVATION |
| Heuristic: similar name/size for delta | Pro Git 10.4 | OBSERVATION |
| Index (staging) as binary file | Pro Git 10.2, `git update-index` | FACT |
| Smart protocol negotiation | Pro Git 10.6 | FACT |
| Thin packs (deltas vs client objects) | Pro Git 10.6 | FACT |
| Bundle files for transfer | Pro Git 7.12 | FACT |
| Reachability determines GC | Pro Git 10.4 | FACT |
| Repacking rewrites delta chains | Pro Git 10.4 | FACT |
| SHA-1 collision vulnerability | SHAttered attack (2017) | FACT |
| Git migrating to SHA-256 | Git v2.29+ release notes | FACT |

---

*Research conducted per Prime methodology: primary sources first (Git source, Pro Git book), evidence over assumptions, distinguish confidence levels.*