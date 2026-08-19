Reusable search research:

- Search algorithm libraries that could be adapted for Prime:
  - FAISS (Facebook AI Similarity Search): ANN (approximate nearest neighbor) vector search
    - Usefulness: Prime can use FAISS for semantic vector search (semantic search mode)
    - Integration: Provide vector embeddings as input; FAISS returns nearest neighbors
  - Annoy (Approximate Nearest Neighbors Oh Yeah): Library by Spotify for ANN search
    - Usefulness: Prime can use Annoy as lighter-weight alternative to FAISS
    - Tradeoff: Fewer features, possibly lower quality ANN than FAISS
  - ScaNN (Scalable Nearest Neighbor Search): Google's library, optimized for large scale
    - Usefulness: Prime can use ScaNN for very large vector indexes (millions of symbols)
    - Integration: Provide vector embeddings; ScaNN returns approximate nearest neighbors
  - Elasticsearch: Full-text search engine with vector similarity (hybrid search)
    - Usefulness: Prime can use Elasticsearch for hybrid search (lexical + semantic)
    - Integration: Index knowledge in Elasticsearch; query via LSP-like API
  - Whoosh: Pure Python search engine library
    - Usefulness: Prime could use Whoosh for lexical search (pure Python, no external deps)
    - Tradeoff: Less scalable than Elasticsearch/FAISS

- Search algorithm adaptation for Prime:
  - Lexical search (inverted index): Adapt existing inverted index implementations
    - Prime's inverted index: term → posting list (symbol locations, confidence, provenance)
    - Usefulness: Standard lexical search for agent queries (find symbols by name)
  - Semantic search (vector search): Adapt ANN library (FAISS/Annoy/ScaNN)
    - Prime's vector embeddings: symbol/type/relationship embeddings in dense vector space
    - Usefulness: Agent finds semantically similar code even with different naming
  - Hybrid search (lexical + semantic): Combine inverted index + ANN search + re-ranking
    - Two-phase retrieval: Retrieve from both indexes, re-rank by combined score
    - Usefulness: Best of both approaches (precision of lexical, recall of semantic)
    - Re-ranking: Linear combination, learning-to-rank, or feedback-driven

- Search result re-ranking:
  - Combine lexical score (TF-IDF, exact match) + semantic score (cosine similarity) + structural score (graph centrality, importance)
  - Usefulness: Present most relevant results first (token efficiency: fewer tokens for agent to find what needs)
  - Re-ranking algorithms: Linear combination, Learning-to-rank (LambdaMART), ListNet, ListMLE
  - Usefulness: Agent consumes fewer tokens (best results at top)

- Search index adaptation:
  - Inverted index: Standard term→posting list; add fields for confidence, provenance, language tag
  - Vector index: ANN library index (FAISS index flat, IVF, HNSW); store vector embeddings per symbol
  - Usefulness: Two-index approach (lexical + semantic) supports hybrid search

- Reusable search components:
  - Inverted index constructor/manager (term → posting list with confidence/provenance)
  - Vector embeddings storage (per-symbol embedding vectors)
  - ANN search library wrapper (FAISS/Annoy/ScaNN integration)
  - Hybrid search re-ranking module (combine lexical + semantic + structural scores)
  - Query processor (parse agent query → lexical/semantic/hybrid search + re-ranking)
  - Result formatter (format retrieved knowledge for agent consumption, token-efficient layout)