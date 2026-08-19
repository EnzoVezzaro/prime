Symbol Indexes research:

- Symbol lookup: Efficient retrieval of symbol information by name or identifier:
  - Hash table: O(1) expected lookup by symbol hash
  - Usefulness: High (direct symbol resolution)
  - Memory overhead: Moderate (hash table + entries)
  - Collision handling: Chaining or open addressing
  - Prime relevance: Core index for agent symbol resolution

- Inverted index (symbol → references/definitions): inverted index mapping symbols to their locations:
  - Forward index: symbol name → list of definition locations
  - Usefulness: High (find all definitions of a symbol)
  - Construction: One pass over indexed data
  - Memory: Inverted index entries + symbol identifiers
  - Prime relevance: Core index for "find all references" queries

- Hybrid symbol index: Combined hash table + inverted index:
  - Hash table for exact symbol name lookup
  - Inverted index for reference/definition discovery
  - Usefulness: High (supports both exact lookup and discovery queries)
  - Memory: Hash table + inverted index entries
  - Prime relevance: Recommended design for comprehensive symbol indexing

- Confidence-annotated symbol index: Each symbol entry includes confidence/provenance:
  - Exact (statically derivable)
  - Inferred (reasonable deduction)
  - Unknown (cannot determine)
  - Usefulness: Agent can weigh reliability of symbol knowledge
  - Prime relevance: Core to graceful degradation across languages

- Hashed symbol identity index: Symbols identified by hash of (name + container + language):
  - Usefulness: Deduplication, content addressing, stability across refactoring
  - Prime relevance: High (connects to Merkle DAG / content addressing research)
  - Tradeoff: Not human-readable, requires lookup table for display

- Fuzzy symbol search: Allowing approximate matches on symbol names:
  - Usefulness: High (typos, renamed symbols, partial name matches)
  - Technique: Levenshtein automata, approximate string matching
  - Tradeoff: Lower precision, higher computational cost
  - Prime relevance: Agent may search with partial/approximate names

- Symbol index for cross-language support:
  - Language-qualified symbol identifiers
  - Mapping between language-specific and universal symbol IDs
  - Usefulness: Agent can find symbols across language boundaries
  - Prime relevance: Core to language-agnosticism research area

- Reusable symbol index components:
  - Hash table implementation
  - Inverted index construction
  - Confidence/provenance annotation schema
  - Hashed identity computation
  - Fuzzy search utility (optional)