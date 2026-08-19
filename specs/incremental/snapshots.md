Snapshots research:

- Immutable snapshots: Complete read-only version of knowledge artifact at a point in time:
  - Usefulness: Agent can snapshot current state, compare across time, derive incrementally from snapshot
  - Tradeoffs: Storage overhead (multiple versions), update requires new snapshot creation

- Snapshot creation:
  - Full snapshot: Capture entire knowledge artifact at point in time
    - Usefulness: Whole-repo state capture (benchmark, comparison, reproduction)
    - Tradeoff: Large storage (entire artifact duplicated)
  - Incremental snapshot: Capture only changes since last snapshot
    - Usefulness: Storage-efficient (only diff stored)
    - Mechanism: Merkle tree diff (changed leaf hashes, new root hash)
    - Tradeoff: More complex to create and derive from

- Snapshot comparison:
  - Root hash comparison: Compare Merkle root hashes between two snapshots
    - Usefulness: Determine if codebase (and knowledge artifact) changed since snapshot
    - Mechanism: Root hash from snapshot A vs. root hash from snapshot B
  - Diff generation: Generate diff of knowledge entries between two snapshots
    - Usefulness: Understand what changed (new symbols, removed symbols, modified relationships)
    - Mechanism: Compare entry sets (union, intersection, difference)
  - Usefulness: Agent can understand codebase evolution

- Incremental derivation from snapshots:
  - Derive new knowledge from base snapshot + incremental changes (deltas)
    - Usefulness: Agent starts from existing snapshot, applies only relevant deltas (fast vs. full re-index)
    - Mechanism: Snapshot + Merkle diff + targeted re-index of changed files
  - Usefulness: Supports agent workflow (focus on changes, not full codebase)

- Snapshot use cases:
  - Codebase evolution tracking: Track how codebase changes over time (who changed what, when)
  - Reproducible analysis: Reproduce analysis results from a point in time (snapshot + analysis tools)
  - Experiment branching: Create snapshot, experiment with changes, compare outcomes
  - Agent state persistence: Persist agent's knowledge state across sessions (snapshot + agent cache)
  - Usefulness: Multiple research and agent use cases

- Snapshot storage considerations:
  - Full snapshots: Artifact-size per snapshot (e.g., 100MB snapshot for 100MB artifact)
  - Incremental snapshots: Diff-size per snapshot (typically much smaller, e.g., 1-10KB per changed file)
  - Compression: Apply ZSTD/other compression to snapshot storage (reduce disk space)
  - Usefulness: Incremental snapshots significantly reduce storage overhead

- Snapshot agent workflow:
  1. Agent starts, loads latest snapshot (or full re-index if no snapshot exists)
  2. Agent explores codebase, makes notes/annotations (stored as incremental deltas)
  3. Agent session ends; snapshots updated (new root hash, deltas recorded)
  4. Next session: loads latest snapshot, applies deltas, continues exploration
  - Usefulness: Agent state persists across sessions; reduces re-indexing overhead

- Snapshot consistency:
  - Immutable: Once created, snapshot never modified (new snapshot for any change)
  - Atomic: Snapshot creation is atomic (either complete or none)
  - Usefulness: Agent can rely on snapshot representing consistent point in time
  - Prime relevance: Supports agent loop and incremental analysis

- Reusable snapshot components:
  - Snapshot creator (full or incremental from Merkle tree)
  - Snapshot comparator (root hash diff, entry set diff)
  - Incremental deriver (apply deltas to base snapshot)
  - Snapshot storer (disk format, compression)
  - Agent delta recorder (agent notes/annotations as deltas)