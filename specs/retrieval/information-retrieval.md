Information Retrieval research:

- Inverted indexes: Inverted mapping from content to its locations:
  - Term → posting list (document/occurrence identifiers)
  - Usefulness: Core for lexical search, symbol search
  - Construction: One pass over data, append occurrence to term's posting list
  - Memory: Term dictionary + posting lists (can be large)
  - Prime relevance: Primary index for lexical agent queries

- Lexical search: Pattern-matching search against indexed content:
  - Exact term match: String equality against indexed terms
  - Prefix matching: Terms starting with prefix (e.g., "User" matches "UserService", "UserRepository")
  - Wildcard support: Single char (_) or multi-char (*) wildcards
  - Usefulness: High (agent searches for symbol names with patterns)
  - Implementation: Trie structure or modified inverted index

- Symbol search: Specialized inverted index for symbols:
  - Index by symbol name, kind, container
  - Usefulness: High (agent finds symbols by name/kind/containment)
  - Difference from general inverted index: Structured symbol attributes, not just raw text

- Structural search: Search against graph structure or annotated structure:
  - Pattern matching against graph topology
  - Usefulness: Medium (specific to graph-query use cases)
  - Examples: "find all cycles", "find all nodes reachable from X", "find all implementations of interface Y"
  - Prime relevance: Agent may need relationship pattern queries

- Semantic search: Search using meaning rather than lexical form:
  - Vector space model: Indexed items represented as dense vectors
  - Query vector: User query or target symbol represented as vector
  - Similarity: Cosine similarity, dot product, or other metric between query and indexed vectors
  - Usefulness: High (agent finds semantically similar code even with different naming conventions)
  - Embedding requirement: Precomputed symbol/relationship embeddings (from analysis phase)
  - Prime relevance: Core to "agent-readable" semantic understanding goal; may be primary search mode

- Vector search: Approximate or exact nearest neighbor search:
  - Exact: Brute-force comparison of query vector against all indexed vectors (O(n) time, precise)
  - Approximate (ANN): Libraries like FAISS, Annoy, ScaNN for fast approximate nearest neighbor (sub-linear time, tiny precision tradeoff)
  - Index structures: KD-tree (low dimensions), ball tree, hierarchical Navigable Small World (HNSW), inverted file (IVF)
  - Usefulness: High (enables semantic search at scale)
  - Prime relevance: Key enabler for large-scale semantic agent retrieval

- Hybrid search: Combining lexical + semantic + structural search:
  - Two-phase retrieval: Retrieve from both lexical and semantic indexes
  - Re-ranking: Combine scores from both modes (e.g., linear combination, learning-to-rank)
  - Reranking algorithms: Learning-to-rank models, feedback-driven re-ranking
  - Usefulness: High (best of both approaches; lexical for precision/recall at top, semantic for breadth)
  - Implementation: Two separate indexes + re-ranking pipeline
  - Prime relevance: Recommended approach for comprehensive agent retrieval

- Ranking: Ordering search results by relevance:
  - TF-IDF: Term frequency-inverse document frequency (lexical relevance weighting)
  - Learning-to-rank: ML model (LambdaMART, ListNet) learns ranking function from query-document pairs
  - PageRank: For graph-structured results (importance-based ranking)
  - Usefulness: High (present best results first to agent, token efficiency)
  - Prime relevance: Critical for agent token efficiency (best results first = fewer tokens needed)

- Filtering: Narrowing search results by metadata constraints:
  - Filter by symbol type (function, class, variable), language, module, tags, confidence level, provenance
  - Usefulness: High (agent queries frequently constrained: "find functions named X in module Y", "find high-confidence references")
  - Index-side filtering: Apply filter during index scan (efficient, reduces result set early)
  - Post-filtering: Retrieve all results then filter (less efficient, but flexible)
  - Prime relevance: Agent queries frequently have constraints; index-side filtering essential for performance

- Faceting: Grouping search results by metadata fields:
  - Show count of results per facet value (e.g., "3 functions, 2 classes, 1 variable" or "5 TypeScript, 3 Python, 1 Go")
  - Usefulness: Medium (helps agent understand search space, refine query, learn codebase structure)
  - Implementation: Additional indexing on metadata fields + count aggregation
  - Prime relevance: Useful for agent to understand codebase structure through search results

- Query expansion: Automatically expanding search query for better recall:
  - Synonym addition: Add related terms based on thesaurus or learned associations
  - Query reformulation: Reformulate based on initial results (pseudo-relevance feedback)
  - Usefulness: Medium (may improve recall at cost of precision; agent may or may not want auto-expansion)
  - Prime relevance: Configurable; agent can choose expansion vs. precise query

- Query refinement: Iterative query improvement based on results:
  - Show preview of results, allow agent to refine based on initial matches
  - Usefulness: High (supports agent interactive exploration of codebase)
  - Prime relevance: Agent loop design; supports progressive disclosure model

- Search precision & recall:
  - Precision: Of retrieved results, what fraction are relevant
  - Recall: Of relevant results, what fraction were retrieved
  - Tradeoff: Typically precision-recall curve; improving one often hurts the other
  - Prime relevance: Agent workload may favor precision@k (first few results) over full recall

- Search index tradeoffs:
  - Precision vs recall: Exact matching vs. broad matching
  - Index size vs query speed: Larger index enables faster queries, smaller index saves memory
  - Static vs dynamic: Pre-built full index vs. incremental update support (rebuilding partial)
  - Prime relevance: Must balance for agent workload (real-time queries, potentially stale data, incremental updates)

- Reusable search index components:
  - Inverted index construction and management
  - Term dictionary implementation
  - Posting list storage and traversal
  - Vector embedding storage and ANN search structure
  - Hybrid search re-ranking pipeline
  - Filtering and faceting infrastructure
  - Query expansion/refinement utilities