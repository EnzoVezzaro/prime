Graph Indexes research:

- Adjacency list index: Representing graph as collection of (node → list of neighbor nodes):
  - Usefulness: Fundamental graph representation for codebase graphs (calls, references, dependencies)
  - Storage: Per-node adjacency list (array, linked list, or vector)
  - Random access: O(1) to get adjacency list of a node, O(deg) to iterate neighbors
  - Reverse lookup: O(n) to find all nodes pointing to a target (requires scanning all lists)
  - Prime relevance: Base structure for codebase relationship representation

- Forward star / Reverse star index:
  - Forward star: For each node, list of outgoing edges (outgoing calls/references)
  - Reverse star: For each node, list of incoming edges (incoming calls/references)
  - Usefulness: High (enables both forward and reverse relationship queries)
  - Storage: Two adjacency structures (double the memory, but enables efficient reverse queries)
  - Prime relevance: Codebase graphs need both direction queries (who calls X? who does X call?)

- CSR/CSC (Compressed Sparse Row/Column) index:
  - Row-pointer format: node i's neighbors stored in contiguous array, row offsets enable O(1) access
  - Usefulness: Efficient storage, fast neighbor iteration
  - Random access: O(1) to start of node's adjacency, O(deg) to iterate
  - Static structure: Requires rebuilding for modifications (incremental update challenge)
  - Prime relevance: For static snapshots of codebase graphs; incremental update pattern needed

- Graph bitmap index: Roaring bitmaps for adjacency representation:
  - Usefulness: Efficient set representation (which nodes are adjacent to a given node)
  - Operations: degree calculation, neighbor iteration, set operations (union, intersection, difference)
  - Compression: Roaring bitmaps automatically compress based on density
  - Random access: Word-level access, neighbor iteration
  - Prime relevance: Codebase graphs often have predictable degree patterns; Roaring bitmaps efficient for moderate-degree nodes

- Eigenvalue/index-free neighborhood: Sampling-based neighborhood approximation:
  - Usefulness: Very large graphs where full neighborhood too expensive
  - Technique: Sample k neighbors, use for similarity, clustering
  - Tradeoff: Approximate, not exact
  - Prime relevance: Large monorepo consideration; approximate may suffice for some agent queries

- Graph indexing for path queries:
  - Indexing shortest paths, reachability, transitive closure
  - Usefulness: Agent queries like "find all symbols reachable from X within N hops"
  - Techniques: Transitive closure index, path indexing, reachability labeling
  - Storage: Significant (transitive closure is O(n²) in worst case)
  - Prime relevance: Agent may need multi-hop relationship queries (e.g., "find all symbols indirectly affected by this change")

- Graph indexing tradeoffs:
  - Exact vs approximate: Exact guarantees vs. space/speed savings
  - Static vs dynamic: Pre-built index vs. incremental update support
  - Query type optimization: Optimize for specific query patterns (neighbor iteration, path queries, degree calculation)
  - Prime relevance: Prime must optimize for agent query patterns (symbol lookup, relationship queries, path traversal)

- Reachable node index: Precomputed set of nodes reachable from each node (within h hops):
  - Usefulness: Fast reachability queries for agent reasoning
  - Storage: O(n * avg_reachable) can be large (transitive closure)
  - Update complexity: High (any graph modification may affect reachability)
  - Prime relevance: For smaller codebases or snapshot-based analysis

- Reusable graph index components:
  - Adjacency list construction
  - CSR/CSC format conversion
  - Roaring bitmap implementation
  - Forward/reverse star structures
  - Path/query pattern indexing utilities