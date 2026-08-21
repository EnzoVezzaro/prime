# Incremental Indexing Synthesis

**Confidence:** OBSERVATION (documented approaches), HYPOTHESIS (Prime recommendations)
**Primary Sources:** Tree-sitter, SCIP, Code-Graph-RAG, Git, Merkle trees
**Last Updated:** August 2026

## Executive Summary

Incremental indexing is critical for interactive agent use. Analysis reveals three viable approaches: file-level invalidation (simple), content-hash based (medium), and Merkle tree (complex). Prime should adopt a **hybrid approach**: file-level with content-hash verification.

## Approaches to Incremental Indexing

### Approach 1: File-Level Invalidation

**Used by:** Code-Graph-RAG, OpenGrok, LSIF

```
1. Detect file change (mtime/size)
2. Re-parse entire file
3. Diff symbols/relationships
4. Apply minimal graph updates
```

| Pros | Cons |
|------|------|
| Simple to implement | Over-re-parses unchanged symbols |
| Works with any parser | No sub-file granularity |
| Well-understood | Cannot detect unchanged functions in changed file |

**Code-Graph-RAG Implementation:**
- File mtime detection
- Full re-parse on change
- Diff symbols against previous version
- Insert/delete/update relationships

**Performance:** 100ms-1s per changed file (depending on file size)

### Approach 2: Content-Hash Based

**Used by:** Git, SCIP (design goal), Prime (planned)

```
1. Compute content hash for each file
2. Compare against stored hash
3. Only re-parse files with changed hashes
4. Optionally: compute per-entity hashes for sub-file granularity
```

| Pros | Cons |
|------|------|
| Avoids unnecessary re-parses | Hash computation overhead |
| Sub-file granularity possible | Requires storing all hashes |
| Content-addressed = cacheable | Cannot detect semantic changes |

**Git's Approach:**
- SHA-1/SHA-256 per file blob
- Tree objects for directory structure
- Commit objects for snapshot
- Packfiles for compression

**Performance:** 10-50ms per changed file (hash comparison + selective re-parse)

### Approach 3: Merkle Tree

**Used by:** IPFS, content-addressed storage, planned for Prime

```
1. Build Merkle tree over file contents
2. Root hash represents entire codebase state
3. Change detection: compare root hashes
4. Invalidation: walk tree to find changed subtrees
5. Re-parse only affected subtrees
```

| Pros | Cons |
|------|------|
| O(log n) change detection | Complex implementation |
| Cryptographic integrity proofs | Higher memory overhead |
| Distributed-friendly | Slower than file-level for single changes |
| Natural cache invalidation | Requires tree maintenance |

**Performance:** O(log n) for change detection, O(changed files) for re-parse

### Approach 4: Tree-sitter Incremental Parsing

**Used by:** Tree-sitter itself, IDE integrations

```
1. Parse initial file → CST
2. On edit: ts_parser_parse_string with edit operation
3. Parser reuses unchanged subtrees
4. ts_tree_get_changed_ranges returns affected byte ranges
5. Map changed ranges to entities
```

| Pros | Cons |
|------|------|
| Sub-function granularity | Syntax-only (no semantic awareness) |
| O(log n) typical complexity | Cannot detect unchanged functions in changed block |
| Built into parser | Requires Tree-sitter parser |

**Performance:** 1-10ms for small edits, 50-100ms for large edits

## Hybrid Approach (Recommended for Prime)

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Incremental Indexer                        │
├─────────────────────────────────────────────────────────────┤
│  Layer 1: File-Level Change Detection                        │
│  - Content hash per file (SHA-256)                           │
│  - Compare against stored hash                               │
│  - Skip unchanged files entirely                             │
├─────────────────────────────────────────────────────────────┤
│  Layer 2: Tree-sitter Incremental Parsing                    │
│  - For changed files, use incremental parse                  │
│  - Get changed ranges from ts_tree_get_changed_ranges        │
│  - Map changed ranges to entity IDs                          │
├─────────────────────────────────────────────────────────────┤
│  Layer 3: Entity-Level Invalidation                          │
│  - For each changed entity:                                  │
│    - Re-extract symbol info                                  │
│    - Re-compute relationships                                │
│    - Update graph nodes/edges                                │
│  - For unchanged entities: skip                              │
├─────────────────────────────────────────────────────────────┤
│  Layer 4: Relationship Re-computation                        │
│  - For changed entities, re-compute affected relationships   │
│  - For unchanged entities, preserve existing relationships   │
└─────────────────────────────────────────────────────────────┘
```

### Implementation Details

#### File-Level Detection

```rust
struct FileChangeDetector {
    hashes: HashMap<PathBuf, ContentHash>,  // path → hash
}

impl FileChangeDetector {
    fn detect_changes(&mut self, root: &Path) -> Vec<PathBuf> {
        let mut changed = Vec::new();
        for entry in WalkDir::new(root) {
            let path = entry.path();
            let hash = ContentHash::from_file(path);
            if self.hashes.get(path) != Some(&hash) {
                changed.push(path.to_path_buf());
                self.hashes.insert(path.to_path_buf(), hash);
            }
        }
        changed
    }
}
```

#### Tree-sitter Incremental Parsing

```rust
struct IncrementalParser {
    parser: ts::Parser,
    trees: HashMap<PathBuf, ts::Tree>,
}

impl IncrementalParser {
    fn parse_incremental(&mut self, path: &Path, source: &[u8]) -> Vec<ChangedRange> {
        let old_tree = self.trees.get(path);
        let new_tree = self.parser.parse(source, old_tree);
        
        let ranges = if let Some(old) = old_tree {
            ts::changed_ranges(old, &new_tree)
        } else {
            vec![ChangedRange::full(source.len())]
        };
        
        self.trees.insert(path.to_path_buf(), new_tree);
        ranges
    }
}
```

#### Entity Invalidation

```rust
struct EntityInvalidator {
    graph: KnowledgeGraph,
}

impl EntityInvalidator {
    fn invalidate(&mut self, file: &Path, ranges: &[ChangedRange]) {
        // Find entities in changed ranges
        let affected = self.graph.entities_in_ranges(file, ranges);
        
        // Remove old relationships
        for entity in &affected {
            self.graph.remove_relationships(entity);
        }
        
        // Re-extract and re-compute
        for entity in &affected {
            let new_info = self.extract_entity(entity);
            self.graph.update_entity(entity, new_info);
            self.recompute_relationships(entity);
        }
    }
}
```

### Performance Expectations

| Scenario | Full Rebuild | Hybrid Approach | Speedup |
|----------|-------------|-----------------|---------|
| Single file change (1K LOC) | 100ms | 10ms | 10x |
| Single function change | 100ms | 2ms | 50x |
| 10 files changed | 1s | 50ms | 20x |
| Rename symbol (100 files) | 10s | 500ms | 20x |

### Storage Requirements

| Component | Size | Notes |
|-----------|------|-------|
| File hashes | ~50 bytes/file | SHA-256 |
| CST cache | ~2-5x source | Tree-sitter trees |
| Entity map | ~100 bytes/entity | ID → position mapping |
| **Total overhead** | ~10-20% | Acceptable for 10x speedup |

## Comparison with Existing Systems

| System | Approach | Granularity | Speed |
|--------|----------|-------------|-------|
| Code-Graph-RAG | File-level (mtime) | File | 100ms-1s |
| SCIP | Per-file document | File | 10-100ms |
| LSIF | Document events | File | 10-100ms |
| OpenGrok | Full reindex | Entire codebase | 30-300s |
| **Prime (hybrid)** | File + Tree-sitter + Entity | Sub-function | 2-10ms |

## Open Questions

1. **OPEN QUESTION:** How to handle cross-file relationship invalidation efficiently? When a type definition changes, how to find all dependent files without scanning entire codebase?

2. **OPEN QUESTION:** Should Prime store CST trees for incremental parsing, or re-parse from source each time (slower but less memory)?

3. **OPEN QUESTION:** How to handle concurrent modifications in multi-agent scenarios? CRDT support for distributed knowledge?

4. **OPEN QUESTION:** What's the optimal hash function for file content? SHA-256 (secure but slow) vs xxHash (fast but collision-prone) vs MetroHash?

## Confidence Assessment

| Claim | Confidence |
|-------|------------|
| File-level invalidation is simplest | **FACT** (documented in 3+ systems) |
| Tree-sitter provides sub-file granularity | **FACT** (API documentation) |
| Hybrid approach achieves 10-50x speedup | **HYPOTHESIS** (requires benchmarking) |
| 10-20% storage overhead is acceptable | **INFERENCE** (based on speedup benefit) |
| Cross-file invalidation is hard | **OBSERVATION** (no system solves perfectly) |
