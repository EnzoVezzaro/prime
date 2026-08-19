Search Indexes research:

- Inverted index: Inverted mapping from content (words, symbols) to its locations in a document:
  - Posting list: For each term, list of document/occurrence identifiers
  - Usefulness: Lexical search, symbol search, keyword search
  - Construction: One pass over indexed data, add occurrence to term's posting list
  - Memory: Term dictionary + posting lists
  - Prime relevance: Core index for lexical agent queries (e.g., "find symbols matching X")

- Lexical search: Pattern-matching search against indexed content:
  - Exact term match, prefix matching, wildcard support
  - Usefulness: High (agent searches for symbol names, patterns)
  - Inverted index essential component
  - Prime relevance: Core agent capability

- Structural search: Search against graph structure or annotated structure:
  - Pattern matching against graph topology (e.g., "find all cycles of length 3")
  - Usefulness: Medium (specific to graph-query use cases)
  - Prime relevance: Agent may need structural pattern queries

- Semantic search: Search using meaning/representation rather than lexical form:
  - Vector space model: Documents/symbols embedded as dense vectors
  - Similarity computation: Cosine similarity, dot product between query and indexed vectors
  - Usefulness: High (agent finds semantically similar code even with different naming)
  - Embedding requirement: Precomputed symbol/relationship embeddings
  - Prime relevance: Emerging capability; central to "agent-readable" knowledge representation goal

- Vector search: Retrieval using vector similarity:
  - Annoy, FAISS, NSNGraph: Approximate nearest neighbor libraries
  - Usefulness: High (semantic similarity search across codebase)
  - Index structure: KD-tree, ball tree, hash-based (for high-dimensional vectors)
  - Approximation: ANNS (Approximate Nearest Neighbor) trades precision for speed
  - Prime relevance: Key enabler for "agent-readable" semantic understanding

- Hybrid search: Combining lexical + semantic + structural search:
  - Re-ranking: Retrieve from both lexical and semantic indexes, re-rank by combined score
  - Usefulness: High (best of both approaches; lexical for precision, semantic for recall/broadening)
  - Implementation: Separate indexes + re-ranking algorithm
  - Prime relevance: Recommended approach for comprehensive agent retrieval

- Graph search: Traversal-based search on graph structures:
  - BFS/DFS from source node
  - Usefulness: Navigating codebase relationships (find all symbols reachable from X)
  - Index support: Precomputed reachability, hub authorities, etc.
  - Prime relevance: Core for agent navigation of codebase knowledge

- Ranking: Ordering search results by relevance:
  - TF-IDF: Term frequency-inverse document frequency (lexical relevance)
  - Learning-to-rank: ML model learned ranking function
  - Usefulness: High (present best results first to agent)
  - Prime relevance: Agent consumption optimized (fewer tokens needed if best results first)

- Filtering: Narrowing search results by metadata constraints:
  - Filter by symbol type, language, module, tags, confidence level
  - Usefulness: High (agent queries often have constraints: "find functions named X in module Y")
  - Implementation: Index-side filtering (efficient) vs. post-filtering (less efficient)
  - Prime relevance: Agent queries frequently constrained

- Faceting: Grouping search results by metadata fields:
  - Show count of results per value (e.g., "3 functions, 2 classes, 1 variable")
  - Usefulness: Medium (helps agent understand search space, refine query)
  - Implementation: Additional indexing on metadata fields
  - Prime relevance: Useful for agent to understand codebase structure through search

- Query expansion: Automatically expanding search query for better recall:
  - Synonym addition, related term addition, query reformulation
  - Usefulness: Medium (may improve recall at cost of precision)
  - Prime relevance: Agent may benefit from automatic expansion for broad searches

- Query refinement: Iterative query improvement based on results:
  - Show preview, allow agent to refine based on initial results
  - Usefulness: High (agent interactive exploration of codebase)
  - Prime relevance: Agent loop design consideration

- Search index tradeoffs:
  - Precision vs recall: Exact matching vs. broad matching
  - Index size vs query speed: Larger index faster queries, smaller index slower
  - Static vs dynamic: Pre-built vs. incremental update support
  - Prime relevance: Must balance for agent workload (real-time queries, potentially stale data)

- Reusable search index components:
  - Inverted index construction
  - Term dictionary implementation
  - Posting list management
  - Vector embedding storage (for semantic search)
  - Hybrid search re-ranking logic
  - Filtering/faceting infrastructure