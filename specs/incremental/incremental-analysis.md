Incremental analysis research:

- Incremental analysis: Updating Prime knowledge artifact when codebase changes without full re-indexing:
  - Core research area (init-promt.md research track #14)
  - Usefulness: Agent productivity (don't halt work on single file change)

- Incremental parsing: Updating syntax tree when source file changes:
  - Tree-sitter: Primary example of incremental parsing library
  - Usefulness: O(log n) or better update time for editor-like keystroke operations
  - Mechanism: Maintain parse state, only re-parse changed regions (differential parsing)
  - Prime relevance: Foundation for incremental indexing (parse only changed files)

- Incremental indexing: Updating knowledge index when source code changes:
  - Content hashing: Hash of file content (or AST) to detect changes
  - Incremental update: Only re-index changed files; keep existing index entries for unchanged files
  - Usefulness: High (avoids full re-index on single file change)
  - Prime relevance: Core mechanism for incremental analysis

- Dependency invalidation: When does a change invalidate existing knowledge?
  - Direct dependency: Changed file symbol → invalidate all knowledge entries referencing that symbol
  - Transitive dependency: Changed file A → invalidate files that import A → invalidate knowledge referencing those
  - Usefulness: Agent knows what to recompute when one file changes
  - Prime relevance: Research question (incremental-analysis.md: "determine what must be recomputed")

- Content hashing: Hash-based change detection:
  - Merkle tree: Hash tree where leaves are file hashes, root is repository root hash
    - Usefulness: Single root hash tracks entire codebase state; change detection via root hash comparison
    - Incremental invalidation: Change in one file produces new root hash; agents detect change via comparison
  - Simple hash: Hash of file content (faster, but only detects whole-file changes)
    - Usefulness: Quick change detection for individual files
  - Prime relevance: Merkle tree for whole-repo; simple hash for individual files

- Merkle trees: Hash tree structure for incremental invalidation:
  - Leaf nodes: File content hashes
  - Internal nodes: Hash of children's hashes
  - Root node: Repository root hash (commits to entire codebase state)
  - Usefulness: Content addressing, incremental invalidation, distributed sharing (CRDTs research area)
  - Prime relevance: Research area (research track #18: cryptography; research track #14: incremental analysis)

- Partial recomputation: What must be recomputed when changes occur:
  - One file changes: Invalidate symbols from that file, recompute references, recompute dependent symbols
  - One symbol changes: Invalidate all knowledge entries referencing that symbol, recompute dependents
  - One dependency changes: Invalidate all symbols that import/depend on changed dependency, recompute
  - Entire package changes: Invalidate all symbols from that package and all dependents
  - Usefulness: Agent knows recomputation scope

- Prime recomputation rules:
  - One file changes: Re-index that file, update root hash, invalidate dependent knowledge entries (transitive closure)
  - One symbol changes: Invalidate that symbol and all knowledge entries that reference or depend on it
  - One dependency changes: Invalidate all symbols that import that dependency, transitively
  - Entire package changes: Invalidate all symbols from that package and all transitive dependents

- Persistent indexes: Indexes saved to disk, loaded on startup:
  - Usefulness: Avoid re-indexing from source on every session start
  - Incremental update: Persistent indexes updated incrementally (not rebuilt from scratch)
  - Invalidation: Stale entries detected via content hashing/Merkle root comparison
  - Prime relevance: Research track #14 (incremental analysis)

- Immutable snapshots:
  - Complete read-only version of knowledge artifact at a point in time
  - Usefulness: Agent can snapshot current state, compare across time, incremental derivation from snapshot
  - Tradeoffs: Storage (multiple versions), update requires new snapshot creation
  - Prime relevance: Research track #14 (incremental analysis): "immutable snapshots"

- Incremental update algorithm:
  1. Content hash each file (or Merkle tree root computation)
  2. Compare new hashes to previous (detect changed files)
  3. Re-index changed files (parse → emit universal knowledge → update index)
  4. Update Merkle tree root (if using)
  5. Invalidate dependent knowledge entries (transitive closure of dependencies)
  6. Prune orphaned entries (symbols no longer in codebase)
  7. Commit increment (atomic or per-structure)

- Reusable incremental analysis components:
  - Content hash computation (per-file, Merkle tree)
  - Change detection (compare new vs. previous hashes)
  - Incremental re-indexing pipeline (parse changed files only)
  - Dependency invalidation engine (transitive closure of dependencies)
  - Persistent index updater (update disk-indexed knowledge incrementally)
  - Snapshot manager (create/compare/derive from snapshots)