Graph Compression research:

- Delta encoding: Storing difference between consecutive values rather than absolute values
  - Usefulness: Good for sorted or sequential data (e.g., sequential symbol IDs)
  - Compression ratio: 2-4x for sequential data
  - CPU cost: Low (subtraction/addition)
  - Random access: Requires decompressing from start or using indexed anchors
  - Prime relevance: Compressing ordered relationship lists

- Adjacency compression: Compressing graph adjacency information:
  - Representing which nodes are connected from each source node
  - Usefulness: Core to codebase graph representation (calls, references, dependencies)
  - Techniques: Forward star, reverse star, compressed sparse row (CSR)
  - Compression ratio: 3-8x depending on graph density
  - CPU cost: Low-Medium
  - Random access: With CSR format, O(1) access to adjacency list of a node
  - Prime relevance: Fundamental for codebase graph compression

- WebGraph: Framework for compressing massive web graphs:
  - Billions of nodes, tens of billions of edges
  - Technique: Converting web graph to compressed form, enabling navigation
  - Compression ratio: Extremely high (billions of edges compressed)
  - CPU cost: Medium (navigation requires decompression)
  - Random access: Limited (navigation via compressed structure)
  - Prime relevance: Idea transfer; codebase graphs more structured than web graphs may allow even more aggressive compression

- Succinct graphs: Graph representation close to information-theoretic lower bound while supporting operations:
  - Core concept: Represent graph using near-minimum bits while supporting degree, neighbor queries
  - Usefulness: Theoretical foundation for efficient graph compression
  - Operations supported: degree(node), neighbor(node, i), select, rank
  - Compression ratio: Near-optimal (close to entropic lower bound)
  - CPU cost: Variable (some operations more expensive than adjacency list)
  - Random access: Supported for degree and neighbor queries
  - Prime relevance: Theoretical target; practical structures (Roaring bitmaps, Elias-Fano) are intermediate

- Compressed sparse representations: Sparse graph representations:
  - Represent only existing edges, implicit nulls for non-edges
  - Usefulness: Sparse codebase graphs (most symbols reference few others)
  - Techniques: Adjacency hash, compressed sparse row/column
  - Compression ratio: Depends on sparsity (high sparsity = high compression)
  - CPU cost: Low-Medium
  - Random access: Depends on specific structure
  - Prime relevance: Codebase graphs are typically sparse (most symbols reference few others)

- Graph compression tradeoffs:
  - Size vs query speed: More compression = slower queries (usually)
  - Random access capability: Some structures support direct lookup, others require decompression
  - Graph density impact: Dense graphs compress less well than sparse graphs
  - Prime relevance: Codebase graphs are typically sparse (most symbols reference few others), which favors compression

- Specific techniques for codebase graphs:
  - Symbol ID compression: Elias-Fano for sorted symbol ID lists
  - Relationship type compression: Small integer encoding (call=1, ref=2, impl=3, etc.)
  - Adjacency list compression: Variable-length encoding per node
  - Degree sequence compression: Most symbols have low degree (few references/calls)

- Reusable graph compression components:
  - Delta encoding utility functions
  - Elias-Fano implementation for sorted integer lists
  - Adjacency list compression/decompression
  - Succinct data structure primitives (rank/select)