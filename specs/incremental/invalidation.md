Invalidation research:

- Invalidation: Marking existing knowledge as stale/obsolete when codebase changes:
  - Core mechanism for incremental analysis (ensures knowledge artifact reflects current codebase)

- Change detection: Identifying which codebase elements have changed:
  - Content hash comparison: New hash vs. stored hash per file (detects whole-file changes)
  - Merkle tree comparison: Root hash comparison (detects any change in codebase)
  - AST comparison: Structural diff of syntax trees (detects precise changes)
  - Usefulness: Fine-grained change detection enables targeted invalidation

- Invalidation strategies:
  - File-level invalidation: Invalidate all knowledge entries referencing changed file
    - Usefulness: Quick, coarse-grained; may invalidate more than necessary
    - Mechanism: File hash change → invalidate all entries with file identifier
  - Symbol-level invalidation: Invalidate knowledge entries referencing changed symbol
    - Usefulness: Targeted; only invalidate affected entries
    - Mechanism: Symbol identifier change → invalidate entries with that symbol ID
  - Relationship-level invalidation: Invalidate entries referencing changed relationships (calls, references, dependencies)
    - Usefulness: Most precise; only invalidate truly affected entries
    - Mechanism: Relationship graph traversal from changed element
  - Usefulness hierarchy: Relationship-level > Symbol-level > File-level (more precise = less wasted recomputation)

- Invalidation propagation:
  - Forward propagation: Change in symbol → invalidate all knowledge that depends ON that symbol (uses of symbol)
  - Backward propagation: Change in symbol → invalidate all knowledge that uses THAT symbol (definitions of symbol)
  - Bidirectional propagation: Both forward and backward (full dependency + use analysis)
  - Usefulness: Direction depends on query type (navigation vs. impact analysis)

- Invalidation confidence decay:
  - Each invalidation cycle may reduce knowledge confidence (if recomputed via heuristic vs. exact re-parse)
  - Usefulness: Track confidence alongside knowledge; mark as "needs re-parse" after N cycles
  - Prime relevance: Graceful degradation (research principle from init-promt.md)

- Invalidation for incremental indexing:
  - while indexing incrementally, ensure new entries don't conflict with existing
  - Use uniqueness checks (symbol ID hash) to prevent duplicates
  - Use version tracking (each knowledge entry has version/stamp) to detect stale entries
  - Usefulness: Maintain index consistency during incremental updates

- Invalidation for Merkle DAGs:
  - Change in one file → new leaf hash → new root hash
  - Agents detect staleness via root hash comparison (no global invalidation pass needed)
  - Usefulness: Scalable invalidation (O(1) root hash comparison vs. O(n) scan)
  - Prime relevance: Research area (content addressing, cryptography research tracks)

- Invalidation failure handling:
  - Stale knowledge served to agent (cache not invalidated properly)
  - Usefulness: Agent may act on outdated information; system must surface staleness marker
  - Prime relevance: Graceful degradation; agent can decide how to handle

- Reusable invalidation components:
  - Change detection engine (content hash/Merkle root comparison)
  - Invalidation strategy selector (file-level/symbol-level/relationship-level)
  - Propagation director (forward/backward/bidirectional invalidation)
  - Confidence tracker (track knowledge confidence across invalidation cycles)
  - Index consistency checker (ensure incremental indexing maintains consistency)
  - Merkle DAG invalidation (root hash comparison for scalability)